use convert_case::{Case, Casing};
use darling::{FromMeta, FromDeriveInput, ast, FromField};
use syn::{parse_macro_input};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(FromMeta)]
struct MacroArgs {
    state: Option<String>,
}

#[derive(FromField, Clone)]
#[darling(attributes(form))]
struct FieldOpts {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    custom: Option<String>,

    #[darling(default)]
    hidden: bool,

    #[darling(default)]
    checkbox: bool,

    #[darling(default)]
    input: bool,

    #[darling(default)]
    textarea: bool,
}

#[derive(FromDeriveInput)]
#[darling(attributes(form), supports(struct_any))]
struct TraitOpts {
    ident: syn::Ident,
    data: ast::Data<(), FieldOpts>,
    state: Option<String>,

    #[darling(default)]
    submit: bool,

    #[darling(default)]
    cancel: bool,

    #[darling(default)]
    enter: bool,
}

#[proc_macro_derive(Form, attributes(form))]
pub fn my_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let opts = TraitOpts::from_derive_input(&parse_macro_input!(input)).unwrap();
    opts.to_token_stream().into()
}

impl ToTokens for FieldOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ty, custom, hidden, checkbox, input, textarea, .. } = self.clone();
        let ident = self.ident.as_ref().unwrap();

        // Check whether the types are supported
        if checkbox && ty != syn::parse_str("bool").unwrap() {
            // println!("{:?} {:?}", self.ty, syn::parse_str::<syn::Ident>("bool").unwrap());
            // compile_error!("Checkbox field can only be used with bool");
        }
        if input && ty != syn::parse_str("String").unwrap() {
            // compile_error!("Input field can only be used with String");
        }
        if textarea && ty != syn::parse_str("String").unwrap() {
            // compile_error!("Textarea field can only be used with String");
        }

        let callback_ident = syn::Ident::new(&format!("{}_cb", ident), ident.span());

        let color = quote!{ color={errors.get(stringify!(#ident)).map(|_| cobul::Color::Danger)} };

        let body = match (hidden, checkbox, input, textarea, &custom) {
            (true, false, false, false, None) => quote! {},
            (false, true, false, false, None) => quote! {
                <cobul::Checkbox input={#callback_ident} checked={#ident} label={stringify!(#ident)} #color />
            },
            (false, false, true, false, None) => quote! {
                <cobul::Input input={#callback_ident} value={#ident} #color />
            },
            (false, false, false, true, None) => quote! {
                <cobul::Textarea input={#callback_ident} value={#ident} #color />
            },
            (false, false, false, false, Some(custom)) => {
                let elem_ident = syn::Ident::new(&custom.to_case(Case::Pascal), ident.span());
                quote! {
                    <#elem_ident input={#callback_ident} value={#ident} state={(*state).clone()}
                    change={change.clone()} submit={submit.clone()} cancel={cancel.clone()}
                    error={errors.get(stringify!(#ident)).cloned()} />
                }
            }
            // _ => compile_error!("must specify a single field type, either checkbox, input, textarea or custom")
            _ => quote! {{"error"}}
        };

        let field_start = match (self.checkbox, &self.custom) {
            (true, None) => quote! { <cobul::simple::Field help={errors.get(stringify!(#ident)).cloned()}> },
            (false, None) => quote! { <cobul::simple::Field label={stringify!(#ident)} help={errors.get(stringify!(#ident)).cloned()}> },
            (_, Some(_)) => quote! {},
        };

        let field_end = match &self.custom {
            None => quote! { </cobul::simple::Field> },
            _ => quote! {},
        };

        let stream = quote! {
            #field_start #body #field_end
        };

        tokens.extend(stream)
    }
}

impl ToTokens for TraitOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fields = match &self.data {
            ast::Data::Struct(fields) => &fields.fields,
            _ => unimplemented!()
        };

        let ident = &self.ident;
        let state = self.state.as_ref().map(|name| syn::Ident::new(name, ident.span()));

        let state_decl = match state {
            Some(state) => quote! { let state = yew::use_state(|| #state::default()); },
            None => quote! { let state = yew::use_state(|| ()); },
        };

        let field_callback = |field: &FieldOpts| {
            let name = field.ident.clone().unwrap();
            let callback = syn::Ident::new(&format!("{}_cb", name), name.span());
            quote! {
                let #callback = {
                    let prev = std::rc::Rc::clone(value);
                    input.reform(move |#name| std::rc::Rc::new(#ident{#name, ..(*prev).clone()}))
                };
            }
        };

        let values = fields.iter().map(|field| field.ident.clone());
        let callbacks = fields.iter().map(field_callback);
        let form_ident = syn::Ident::new(&format!("{}Form", ident), ident.span());

        let enter_cb = match self.enter {
            true => quote! {
                let submit_c = submit.clone();
                let onkeypress = Callback::from(move |e: KeyboardEvent| {
                    if e.key() == "Enter" { submit_c.emit(()) }
                });
            },
            false => quote! {}
        };

        let enter_div = match self.enter {
            true => quote! { <div onkeypress={onkeypress}>},
            false => quote! { <div> }
        };

        let cancel = match self.cancel {
            true => quote! { <cobul::Button color={cobul::Color::Info} light=true click={cancel}> {"Cancel"} </cobul::Button> },
            false => quote! {}
        };

        let submit = match self.submit {
            true => quote! { <cobul::Button color={cobul::Color::Info} click={submit}> {"Submit"} </cobul::Button> },
            false => quote! {}
        };

        let stream = quote! {
            #[derive(yew::Properties, std::cmp::PartialEq)]
            pub struct Props {
                pub value: std::rc::Rc<#ident>,
                pub input: yew::Callback<std::rc::Rc<#ident>>,

                #[prop_or_default]
                pub submit: yew::Callback<()>,

                #[prop_or_default]
                pub cancel: yew::Callback<()>,
            }

            #[yew::function_component(#form_ident)]
            pub fn view(props: &Props) -> yew::Html {
                #state_decl

                let errors: std::collections::HashMap<_, _> = validator::Validate::validate(&*props.value)
                    .err()
                    .unwrap_or_default()
                    .field_errors()
                    .into_iter()
                    .map(|(field, vec)| (field.to_owned(), vec.first().unwrap().to_string()))
                    .collect();

                let state_c = state.clone();
                let change = Callback::from(move |new| state_c.set(new));

                let Props { value, input, submit, cancel } = props.clone();
                let #ident { #(#values),* } = (**value).clone();

                #enter_cb

                #(#callbacks);*

                yew::html! {
                    #enter_div
                    #(#fields)*
                    <cobul::Buttons> #cancel #submit </cobul::Buttons>
                    </div>
                }
            }
        };

        tokens.extend(stream)
    }
}

// #[proc_macro_attribute]
// pub fn your_attr(args: TokenStream, input: TokenStream) -> TokenStream {
//     let attr_args = parse_macro_input!(args as AttributeArgs);
//     let _input = parse_macro_input!(input as ItemFn);
//
//     let args = match MacroArgs::from_list(&attr_args) {
//         Ok(v) => v,
//         Err(e) => { return TokenStream::from(e.write_errors()); }
//     };
//
//     println!("{args:?}");
//
//     TokenStream::new()
// }
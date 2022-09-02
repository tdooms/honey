use convert_case::{Case, Casing};
use darling::{FromMeta, FromDeriveInput, ast, FromField};
use syn::{AttributeArgs, Field, ItemFn, parse_macro_input};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

#[derive(FromMeta)]
struct MacroArgs {
    state: Option<String>,
}

#[derive(FromField)]
#[darling(attributes(form))]
struct FieldOpts {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    custom: Option<String>,

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
}

#[proc_macro_derive(Form, attributes(form))]
pub fn my_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let opts = TraitOpts::from_derive_input(&parse_macro_input!(input)).unwrap();
    opts.to_token_stream().into()
}

impl ToTokens for FieldOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.ident.as_ref().unwrap();

        // Check whether the types are supported
        if self.checkbox && self.ty != syn::parse_str("bool").unwrap() {
            println!("{:?} {:?}", self.ty, syn::parse_str::<syn::Ident>("bool").unwrap());
            // compile_error!("Checkbox field can only be used with bool");
        }
        if self.input && self.ty != syn::parse_str("String").unwrap() {
            // compile_error!("Input field can only be used with String");
        }
        if self.textarea && self.ty != syn::parse_str("String").unwrap() {
            // compile_error!("Textarea field can only be used with String");
        }

        let callback_ident = syn::Ident::new(&format!("{}_cb", ident), ident.span());

        let body = match (self.checkbox, self.input, self.textarea, &self.custom) {
            (true, false, false, None) => quote! {
                <cobul::Checkbox input={#callback_ident} checked={#ident} label={stringify!(#ident)} />
            },
            (false, true, false, None) => quote! {
                <cobul::Input input={#callback_ident} value={#ident} />
            },
            (false, false, true, None) => quote! {
                <cobul::Textarea input={#callback_ident} value={#ident} />
            },
            (false, false, false, Some(custom)) => {
                let elem_ident = syn::Ident::new(&custom.to_case(Case::Pascal), ident.span());
                quote! {<#elem_ident input={#callback_ident} value={#ident} state={state.clone()} />}
            }
            // _ => compile_error!("must specify a single field type, either checkbox, input, textarea or custom")
            _ => quote! {}
        };

        let field_start = match (self.checkbox, &self.custom) {
            (true, None) => quote! { <cobul::simple::Field> },
            (false, None) => quote! { <cobul::simple::Field label={stringify!(#ident)}> },
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

        let field_callback = |field: &FieldOpts| {
            let name = field.ident.clone().unwrap();
            let callback = syn::Ident::new(&format!("{}_cb", name), name.span());
            quote! {
                let #callback = {
                    let prev = std::rc::Rc::clone(value);
                    change.reform(move |#name| std::rc::Rc::new(#ident{#name, ..(*prev).clone()}))
                };
            }
        };

        let values = fields.iter().map(|field| field.ident.clone());
        let callbacks = fields.iter().map(field_callback);
        let form_ident = syn::Ident::new(&format!("{}Form", ident), ident.span());

        let stream = quote! {
            #[derive(yew::Properties, std::cmp::PartialEq)]
            pub struct Props {
                pub value: std::rc::Rc<#ident>,
                pub change: yew::Callback<std::rc::Rc<#ident>>,

                #[prop_or_default]
                pub submit: yew::Callback<std::rc::Rc<#ident>>,
            }

            #[yew::function_component(#form_ident)]
            pub fn view(props: &Props) -> yew::Html {
                let state = yew::use_state(|| #state);

                let Props { value, change, submit } = props.clone();
                let #ident { #(#values),* } = (**value).clone();

                #(#callbacks);*

                yew::html! { <> #(#fields)* </> }
                // html!{}
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
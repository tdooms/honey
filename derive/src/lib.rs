use darling::{FromMeta, FromDeriveInput, ast, FromField};
use syn::{AttributeArgs, Field, ItemFn, parse_macro_input};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Debug, Default, Clone, FromMeta)]
#[darling(default)]
enum FormFieldKind {
    #[default]
    Input,
    TextArea,
    Checkbox,
    // Custom(Box<dyn Fn()>)
}

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
    field: FormFieldKind,
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
        let ty = &self.ty;

        let body = match self.field {
            FormFieldKind::Input => quote!{ <cobul::Input oninput={yew::Callback::noop()} /> },
            FormFieldKind::TextArea => quote!{ <cobul::Textarea oninput={yew::Callback::noop()} /> },
            FormFieldKind::Checkbox => quote!{ <cobul::Checkbox id="aargh" label="label" onchange={yew::Callback::noop()} /> }
        };

        let stream = quote! {
            <cobul::simple::Field label={stringify!(#ident)}> #body </cobul::simple::Field>
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
        let stream = quote! {
            impl #ident {
                pub fn view(&self) -> yew::Html {
                    yew::html! { <> #(#fields)* </> }
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
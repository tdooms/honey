use darling::{FromMeta, FromDeriveInput, ast, FromField};
use syn::{AttributeArgs, ItemFn, parse_macro_input};
use proc_macro::TokenStream;


#[derive(Debug, Default, Clone, Copy, FromMeta)]
#[darling(default)]
enum Volume {
    #[default]
    Normal,
    Whisper,
    Shout,
}


#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default)]
    timeout_ms: Option<u16>,
    path: String,
}

#[derive(Default, FromMeta, Debug)]
#[darling(default)]
struct Lorem {
    #[darling(rename = "sit")]
    ipsum: bool,
    dolor: Option<String>,
}

#[derive(Debug, FromField)]
#[darling(attributes(my_trait))]
struct MyFieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    volume: Option<Volume>,
}

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(my_trait), supports(struct_any))]
struct MyTraitOpts {
    ident: syn::Ident,
    data: ast::Data<(), MyFieldReceiver>,
    lorem: Lorem,
}

#[proc_macro_derive(MyTrait, attributes(my_trait))]
pub fn my_trait(input: TokenStream) -> TokenStream {
    let opts = MyTraitOpts::from_derive_input(&parse_macro_input!(input)).unwrap();

    match opts.data {
        ast::Data::Struct(fields) => {
            println!("{:?}", fields.fields[0].volume)
        },
        _ => unimplemented!()
    }


    TokenStream::new()
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
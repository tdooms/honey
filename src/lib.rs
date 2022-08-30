use darling::FromMeta;
use syn::{AttributeArgs, ItemFn, parse_macro_input};
use proc_macro::TokenStream;

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default)]
    timeout_ms: Option<u16>,
    path: String,
}

#[proc_macro_attribute]
pub fn your_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let _input = parse_macro_input!(input as ItemFn);

    let args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };

    println!("{args:?}");

    TokenStream::new()
}
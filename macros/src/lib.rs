use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn preview(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let ident = item.sig.ident.clone();
    let block = item.block.clone();
    let vis = item.vis.clone();

    let s = ident.to_string();
    let name = s.strip_suffix("Preview").unwrap_or(&s);


    let expanded = quote! {
        #[allow(non_upper_case_globals)]
        #vis static #ident: lookbook::Preview = lookbook::Preview::new(#name, |cx| #block);
    };
    expanded.into()
}

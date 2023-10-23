use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, PatType};

#[proc_macro_attribute]
pub fn preview(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let ident = item.sig.ident.clone();
    let block = item.block.clone();
    let vis = item.vis.clone();

    let s = ident.to_string();
    let name = s.strip_suffix("Preview").unwrap_or(&s);

    let mut states = Vec::new();
    let mut from_states = Vec::new();
    let mut controls = Vec::new();
    for input in item.sig.inputs.into_iter().skip(1) {
        match input {
            FnArg::Typed(typed) => {
                let ty = typed.ty;
                let pat = typed.pat;
                
                states.push(quote!(let #pat = use_state(cx, || <#ty>::state());));
                
                from_states.push(quote!(let #pat = <#ty>::from_state(cx, &**#pat);));

                controls.push(quote!(<#ty>::control(cx, #pat),));
            }
            _ => todo!(),
        }
    }


    let expanded = quote! {
        #[allow(non_upper_case_globals)]
        #vis static #ident: lookbook::Preview = lookbook::Preview::new(#name, |cx| {
            use dioxus::prelude::*;
            use lookbook::Stateful;

            fn f<'a>(cx: Scope<'a>) -> Element<'a> {
                #(#states)*

                let controls = render!(
                    #(#controls)*
                );

                #(#from_states)*

                render!(
                    lookbook::Look { name: #name, controls: controls,
                        #block
                    }
                )
            }
            f(cx)
        });
    };
    expanded.into()
}

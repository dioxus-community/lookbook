use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, token, Attribute, Expr, FnArg, ItemFn, Lit};

fn collect_docs(attrs: &[Attribute]) -> String {
    let mut docs = String::new();
    for attr in attrs {
        if attr.path().get_ident().unwrap().to_string() == "doc" {
            let meta = attr.meta.require_name_value().unwrap();
            if let Expr::Lit(expr) = &meta.value {
                if let Lit::Str(lit) = &expr.lit {
                    docs.push_str(&lit.value());
                    docs.push('\n');
                }
            }
        }
    }
    docs
}

#[proc_macro_attribute]
pub fn preview(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);

    let docs = collect_docs(&item.attrs);

    let ident = item.sig.ident.clone();
    let block = item.block.clone();
    let vis = item.vis.clone();

    let s = ident.to_string();
    let name = s.strip_suffix("Preview").unwrap_or(&s);

    let mut states = Vec::new();
    let mut from_states = Vec::new();
    let mut controls = Vec::new();
    for arg in item.sig.inputs.into_iter().skip(1) {
        match arg {
            FnArg::Typed(typed) => {
                let docs = collect_docs(&typed.attrs);

                let ty = typed.ty;
                let pat = typed.pat;

                states.push(quote!(let #pat = use_state(cx, || <#ty>::state());));
                from_states.push(quote!(let #pat = <#ty>::from_state(cx, &**#pat);));

                controls.push(quote!(div {
                    <#ty>::control(cx, #pat)
                    p { #docs },
                }));
            }
            _ => todo!(),
        }
    }

    let controls = render_with_location(quote!(#(#controls)*), name, 0);

    let look = render_with_location(
        quote!(
            lookbook::Look { name: #name, docs: #docs, controls: controls,
                #block
            }
        ),
        name,
        1,
    );

    let expanded = quote! {
        #[allow(non_upper_case_globals)]
        #vis static #ident: lookbook::Preview = lookbook::Preview::new(#name, |cx| {
            use dioxus::prelude::*;
            use lookbook::Stateful;

            fn f<'a>(cx: Scope<'a>) -> Element<'a> {
                #(#states)*

                let controls = cx.render(#controls);

                #(#from_states)*

                cx.render(#look)
            }
            f(cx)
        });
    };
    expanded.into()
}

fn render_with_location(
    tokens: proc_macro2::TokenStream,
    name: &str,
    idx: u8,
) -> proc_macro2::TokenStream {
    let location = format!("__lookbook/{name}.rs:0:0:{idx}");
    let rsx: dioxus_rsx::CallBody = syn::parse2(tokens).unwrap();
    rsx.render_with_location(location)
}

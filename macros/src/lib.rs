use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, Expr, FnArg, ItemFn, Lit, Meta};

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
            FnArg::Typed(typed_arg) => {
                let mut docs = String::new();
                let mut default = quote!(None);

                for attr in typed_arg.attrs {
                    let path = attr.path().get_ident().unwrap().to_string();
                    if path == "doc" {
                        let meta = attr.meta.require_name_value().unwrap();
                        if let Expr::Lit(expr) = &meta.value {
                            if let Lit::Str(lit) = &expr.lit {
                                docs.push_str(&lit.value());
                                docs.push('\n');
                            }
                        }
                    } else if path == "lookbook" {
                        let path = attr.meta.require_list().unwrap();
                        let meta: Meta = syn::parse2(path.tokens.clone()).unwrap();

                        if let Meta::NameValue(meta_name_value) = meta {
                            if meta_name_value.path.is_ident("default") {
                                let value = meta_name_value.value;
                                default = quote!(Some(#value));
                            }
                        }
                    }
                }

                let ty = typed_arg.ty;
                let pat = typed_arg.pat;
                let pat_name = pat.to_token_stream().to_string();

                states.push(quote!(let #pat = use_state(cx, || <#ty>::state(#default));));
                from_states.push(quote!(let #pat = <#ty>::from_state(cx, &**#pat);));

                controls.push(quote!(div {
                    display: "flex",
                    flex_direction: "column",
                    <#ty>::control(cx, #pat_name, #pat)
                    p { #docs },

                    div {
                        dioxus_material::Chip {
                            onclick: |_| {

                            },
                            span { "Default: " }
                            code { #default }
                        }
                    }
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

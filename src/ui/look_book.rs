use crate::{prefixed_route::use_prefix, register, PrefixedRoute, Preview};
use dioxus::prelude::*;
use dioxus_material::{use_theme_provider, Theme};
use dioxus_router::prelude::Router;

#[component]
pub fn LookBook<I: IntoIterator<Item = Preview> + Clone>(
    cx: Scope,
    previews: I,
    prefix: Option<&'static str>,
) -> Element {
    use_theme_provider(cx, Theme::default());

    for preview in previews.clone() {
        register(preview.name, preview.component)
    }

    use_prefix(cx, *prefix);

    render! { Router::<PrefixedRoute> {} }
}

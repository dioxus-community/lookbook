use crate::{prefixed_route::use_prefix, PrefixedRoute};
use dioxus::prelude::*;
use dioxus_material::{use_theme_provider, Theme};
use dioxus_router::prelude::Router;

#[component]
pub fn LookBook(cx: Scope, prefix: Option<&'static str>) -> Element {
    use_theme_provider(cx, Theme::default());

    use_prefix(cx, *prefix);

    render! { Router::<PrefixedRoute> {} }
}

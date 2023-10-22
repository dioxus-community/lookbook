use crate::{prefixed_route::use_prefix, register, PrefixedRoute, Preview, HOME};
use dioxus::prelude::*;
use dioxus_material::Theme;
use dioxus_router::prelude::Router;

#[component]
pub fn LookBook<I: IntoIterator<Item = Preview> + Clone>(
    cx: Scope,
    previews: I,
    home: Component,
    prefix: Option<&'static str>,
) -> Element {
    use_effect(cx, (), move |()| {
        for preview in previews.clone() {
            register(preview.name, preview.component)
        }

        HOME.try_with(|cell| *cell.borrow_mut() = Some(*home))
            .unwrap();

        async move {}
    });

    use_prefix(cx, *prefix);

    render! {
        Theme { Router::<PrefixedRoute> {} }
    }
}

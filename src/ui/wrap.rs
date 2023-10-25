use crate::{prefixed_route::PrefixedRoute, ui::pane::HorizontalPane, Route, CONTEXT};
use dioxus::prelude::*;
use dioxus_material::{use_theme, Icon, IconFont, IconKind};
use dioxus_router::prelude::*;

/// The main application wrap component.
#[component]
pub fn Wrap(cx: Scope) -> Element {
    let query = use_state(cx, || String::new());
    let elements = use_memo(cx, query, move |_| {
        CONTEXT
            .try_with(|cx| {
                cx.borrow()
                    .iter()
                    .filter(|(name, _)| name.to_lowercase().contains(&query.to_lowercase()))
                    .copied()
                    .collect::<Vec<_>>()
            })
            .unwrap()
    });

    let navigator = use_navigator(cx);
    let theme = use_theme(cx);

    let left = render!(
        div {
            flex: 1,
            display: "flex",
            flex_direction: "column",
            gap: "10px",
            width: "200px",
            margin: 0,
            padding: "10px 20px",
            font_size: "14px",
            background: "#eeeeee",
            div {
                display: "flex",
                flex_direction: "row",
                align_items: "center",
                justify_content: "space-between",
                margin: "20px 0",
                h1 {
                    cursor: "pointer",
                    margin: "0",
                    onclick: |_| {
                        navigator.push(Route::Home);
                    },
                    "Lookbook"
                }
                Icon { kind: IconKind::Settings }
            }
            input {
                placeholder: "Search",
                value: "{query}",
                width: "100%",
                margin: "5px",
                margin_bottom: "20px",
                padding: "10px",
                border: "2px solid #999",
                border_radius: &*theme.border_radius_small,
                outline: "none",
                background: "none",
                font_size: 14.,
                oninput: move |event: FormEvent| query.set(event.value.clone())
            }
            elements.into_iter().map(move | (name, _) | { render!(NavItem { route :
            Route::ComponentScreen { name : name.to_string(), }, label : "{name}" }) })
        }
    );

    let right = render!( Outlet::<PrefixedRoute> {} );

    cx.render(rsx! {
        IconFont {}
        div {
            position: "absolute",
            top: 0,
            left: 0,
            width: "100vw",
            height: "100vh",
            display: "flex",
            flex_direction: "row",
            font_family: "sans-serif",
            margin: 0,
            padding: 0,
            HorizontalPane { left: left, right: right }
        }
    })
}

/// Navigation rail item component.
#[component]
fn NavItem<'a>(cx: Scope<'a>, route: Route, label: &'a str) -> Element<'a> {
    let navigator = use_navigator(cx);
    let current_route: Option<PrefixedRoute> = use_route(cx);
    let theme = use_theme(cx);

    let prefixed_route = PrefixedRoute(route.clone());
    let is_selected = current_route.as_ref() == Some(&prefixed_route);

    render!(
        div {
            padding: "10px 15px",
            border_radius: &*theme.border_radius_small,
            cursor: "pointer",
            background: if is_selected { &theme.secondary_container_color } else { "" },
            onclick: |_| {
                navigator.push(PrefixedRoute(route.clone()));
            },
            "{label}"
        }
    )
}

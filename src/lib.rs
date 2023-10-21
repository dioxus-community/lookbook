use dioxus::prelude::*;
use dioxus_material::{use_theme, use_theme_provider, Theme};
use dioxus_router::prelude::*;

thread_local! {
    static CONTEXT: RefCell<Vec<(&'static str, Component)>>= RefCell::new(Vec::new());
}

pub fn register(name: &'static str, component: Component) {
    CONTEXT
        .try_with(|cx| cx.borrow_mut().push((name, component)))
        .unwrap();
}

#[component]
pub fn LookBook<'a>(cx: Scope<'a>) -> Element<'a> {
    use_theme_provider(cx, Theme::default());

    render! { Router::<Route> {} }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(Nav)]
    #[route("/")]
    Home,
    #[route("/:name")]
    ComponentScreen { name: String },
}

#[component]
fn Home(cx: Scope) -> Element {
    render! {"Home"}
}

#[component]
fn ComponentScreen(cx: Scope, name: String) -> Element {
    let (_name, Child) = CONTEXT
        .try_with(|cx| cx.borrow().iter().find(|(n, _)| n == name).unwrap().clone())
        .unwrap();

    render! { Child {} }
}

#[component]
fn Nav(cx: Scope) -> Element {
    let elements = CONTEXT.try_with(|cx| cx.borrow().clone()).unwrap();

    cx.render(rsx! {
        div {
            width: "100vw",
            height: "100vh",
            display: "flex",
            flex_direction: "row",
            font_family: "sans-serif",
            margin: 0,
            padding: 0,
            div {
                display: "flex",
                flex_direction: "column",
                gap: "10px",
                width: "200px",
                margin: 0,
                padding: "10px 5px",
                font_size: "14px",
                background: "#eeeeee",
                elements.into_iter().map(move |(name, _)|  {
                    render!(NavItem {
                        route: Route::ComponentScreen {
                            name: name.to_string(),
                        },
                        label: "{name}"
                    })
                })
            }
            Outlet::<Route> {}
        }
    })
}

#[component]
fn NavItem<'a>(cx: Scope<'a>, route: Route, label: &'a str) -> Element<'a> {
    let navigator = use_navigator(cx);
    let current_route: Option<Route> = use_route(cx);
    let theme = use_theme(cx);

    let is_selected = current_route.as_ref() == Some(route);
    render!(
        div {
            padding: "10px 15px",
            border_radius: &*theme.border_radius,
            cursor: "pointer",
            background: if is_selected { &theme.secondary_container_color } else { "" },
            onclick: |_| {
                navigator
                    .push(Route::ComponentScreen {
                        name: label.to_string(),
                    });
            },
            "{label}"
        }
    )
}

#[component]
pub fn Look<'a>(
    cx: Scope<'a>,
    name: &'static str,
    controls: Element<'a>,
    children: Element<'a>,
) -> Element<'a> {
    render!(
        div { flex: 1, display: "flex", flex_direction: "column",
            div { flex: 1, display: "flex", flex_direction: "column", padding: "20px",
                h2 { "{name}" }
                div { flex: 1, children }
            }
            div { flex: 1, max_height: "400px", overflow_y: "auto", padding: "20px", background: "#f9f9f9",
                h4 { "Controls" }
                controls
            }
        }
    )
}

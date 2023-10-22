use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod ui;
use ui::Nav;
pub use ui::{Look, LookBook};

mod prefixed_route;
pub(crate) use prefixed_route::PrefixedRoute;

thread_local! {
    static CONTEXT: RefCell<Vec<(&'static str, Component)>>= RefCell::new(Vec::new());
}

pub fn register(name: &'static str, component: Component) {
    CONTEXT
        .try_with(|cx| cx.borrow_mut().push((name, component)))
        .unwrap();
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
    #[allow(non_snake_case)]
    let (_name, Child) = CONTEXT
        .try_with(|cx| cx.borrow().iter().find(|(n, _)| n == name).unwrap().clone())
        .unwrap();

    render!(Child {})
}

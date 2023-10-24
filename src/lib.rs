use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub use lookbook_macros::preview;

mod ui;
use ui::Wrap;
pub use ui::{Look, LookBook};

mod prefixed_route;
pub(crate) use prefixed_route::PrefixedRoute;

#[derive(Clone, Copy)]
pub struct Preview {
    name: &'static str,
    component: Component,
}

impl Preview {
    pub const fn new(name: &'static str, component: Component) -> Self {
        Self { name, component }
    }
}

thread_local! {
    static CONTEXT: RefCell<Vec<(&'static str, Component)>>= RefCell::new(Vec::new());

    static HOME: RefCell<Option<Component>> = RefCell::new(None);
}

pub fn register(name: &'static str, component: Component) {
    CONTEXT
        .try_with(|cx| cx.borrow_mut().push((name, component)))
        .unwrap();
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(Wrap)]
    #[route("/")]
    Home,
    #[route("/:name")]
    ComponentScreen { name: String },
}

#[component]
fn Home<'a>(cx: Scope<'a>) -> Element<'a> {
    #[allow(non_snake_case)]
    let Child = HOME
        .try_with(|cell| cell.borrow().clone().unwrap())
        .unwrap();
    render!(Child {})
}

#[component]
fn ComponentScreen(cx: Scope, name: String) -> Element {
    #[allow(non_snake_case)]
    let (_name, Child) = CONTEXT
        .try_with(|cx| cx.borrow().iter().find(|(n, _)| n == name).unwrap().clone())
        .unwrap();

    render!(Child {})
}

pub trait Stateful<'a> {
    type State;

    fn state() -> Self::State;

    fn from_state<T>(cx: Scope<'a, T>, state: &Self::State) -> Self;

    fn control(cx: Scope<'a>, state: &'a UseState<Self::State>) -> Element<'a>;
}

impl<'a> Stateful<'a> for &'a str {
    type State = String;

    fn state() -> Self::State {
        String::new()
    }

    fn from_state<T>(cx: Scope<'a, T>, state: &Self::State) -> Self {
        cx.bump().alloc(state.clone())
    }

    fn control(cx: Scope<'a>, state: &'a UseState<Self::State>) -> Element<'a> {
        render!(dioxus_material::TextField {
            label: "Label",
            value: state,
            onchange: move |event: FormEvent| state.set(event.data.value.clone())
        })
    }
}

impl<'a> Stateful<'a> for u32 {
    type State = u32;

    fn state() -> Self::State {
        0
    }

    fn from_state<T>(_cx: Scope<'a, T>, state: &Self::State) -> Self {
        *state
    }

    fn control(cx: Scope<'a>, state: &'a UseState<Self::State>) -> Element<'a> {
        render!(dioxus_material::TextField {
            label: "Label",
            value: "{state}",
            onchange: move |event: FormEvent| state.set(event.data.value.parse().unwrap())
        })
    }
}

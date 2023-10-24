use dioxus::prelude::*;

/// A controllable property.
pub trait Control<'a>: Sized {
    type State;

    /// Create the initial state.
    fn state(default: Option<Self>) -> Self::State;

    /// Convert the current state to `Self`.
    fn from_state<T>(cx: Scope<'a, T>, state: &Self::State) -> Self;

    /// Render the controller element.
    fn control(cx: Scope<'a>, name: &'static str, state: &'a UseState<Self::State>) -> Element<'a>;
}

impl<'a> Control<'a> for &'a str {
    type State = String;

    fn state(default: Option<Self>) -> Self::State {
        default.map(String::from).unwrap_or_default()
    }

    fn from_state<T>(cx: Scope<'a, T>, state: &Self::State) -> Self {
        cx.bump().alloc(state.clone())
    }

    fn control(cx: Scope<'a>, name: &'static str, state: &'a UseState<Self::State>) -> Element<'a> {
        render!(dioxus_material::TextField {
            label: name,
            value: state,
            onchange: move |event: FormEvent| state.set(event.data.value.clone())
        })
    }
}

impl<'a> Control<'a> for u32 {
    type State = u32;

    fn state(_default: Option<Self>) -> Self::State {
        0
    }

    fn from_state<T>(_cx: Scope<'a, T>, state: &Self::State) -> Self {
        *state
    }

    fn control(cx: Scope<'a>, name: &'static str, state: &'a UseState<Self::State>) -> Element<'a> {
        render!(dioxus_material::TextField {
            label: name,
            value: "{state}",
            onchange: move |event: FormEvent| state.set(event.data.value.parse().unwrap())
        })
    }
}

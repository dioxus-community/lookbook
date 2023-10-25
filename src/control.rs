use std::fmt::Arguments;

use dioxus::prelude::*;
use dioxus_material::use_theme;
use serde::{Deserialize, Serialize};

/// A controllable property.
pub trait Control<'a>: Sized {
    type State;

    /// Create the initial state.
    fn state(default: Option<impl Into<Self>>) -> Self::State;

    /// Convert the current state to `Self`.
    fn from_state<T>(cx: Scope<'a, T>, state: &Self::State) -> Self;

    /// Render the controller element.
    fn control(cx: Scope<'a>, name: &'static str, state: &'a UseState<Self::State>) -> Element<'a>;
}

impl<'a> Control<'a> for &'a str {
    type State = String;

    fn state(default: Option<impl Into<Self>>) -> Self::State {
        default.map(Into::into).map(String::from).unwrap_or_default()
    }

    fn from_state<T>(cx: Scope<'a, T>, state: &Self::State) -> Self {
        cx.bump().alloc(state.clone())
    }

    fn control(cx: Scope<'a>, name: &'static str, state: &'a UseState<Self::State>) -> Element<'a> {
        render!(
            Input {
                value: &***state,
                oninput: move |event: FormEvent| state.set(event.data.value.clone())
            }
        )
    }
}


#[derive(Default)]
pub struct Json<T>(pub T);

impl<T> From<T> for Json<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<'a, T> IntoDynNode<'a> for Json<T>
where
    T: Clone + Default + Deserialize<'a> + Serialize,
{
    fn into_vnode(self, cx: &'a ScopeState) -> dioxus::core::DynamicNode<'a> {
        let s = serde_json::to_string(&self.0).unwrap();
        cx.make_node(s)
    }
}

impl<'a, T> Control<'a> for Json<T>
where
    T: Clone + Default + Deserialize<'a> + Serialize,
{
    type State = T;

    fn state(default: Option<impl Into<Self>>) -> Self::State {
        default.map(Into::into).unwrap_or_default().0
    }

    fn from_state<U>(cx: Scope<'a, U>, state: &Self::State) -> Self {
        Self(state.clone())
    }

    fn control(cx: Scope<'a>, name: &'static str, state: &'a UseState<Self::State>) -> Element<'a> {
        let json = serde_json::to_string(&**state).unwrap();

        render!(
            Input {
                value: "{json}",
                oninput: move |event: FormEvent| {
                    let value = cx.bump().alloc(event.data.value.clone());
                    if let Ok(new_state) = serde_json::from_str(value) {
                        state.set(new_state);
                    }
                }
            }
        )
    }
}

#[component]
fn Input<'a>(cx: Scope<'a>, value: &'a str, oninput: EventHandler<'a, FormEvent>) -> Element<'a> {
    let theme = use_theme(cx);

    render!(
        input {
            border: "2px solid #e7e7e7",
            padding: "10px",
            border_radius: &*theme.border_radius_small,
            font_size: "{theme.label_small}px",
            outline: "none",
            background: "none",
            value: *value,
            oninput: move |event| oninput.call(event)
        }
    )
}

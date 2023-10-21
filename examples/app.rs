use dioxus::prelude::*;
use dioxus_material::{Button, TextField};
use lookbook::{register, Look, LookBook};

#[component]
fn ButtonPage(cx: Scope) -> Element {
    let label = use_state(cx, || String::from("Filled Button"));

    render!(
        Look {
            name: "Button",
            controls: render!(
                TextField { label : "Label", value : label, onchange : move | event : FormEvent |
                label.set(event.data.value.clone()) }
            ),
            Button { onclick: |_| {}, &*** label }
        }
    )
}

#[component]
fn TextButtonPage(cx: Scope) -> Element {
    let label = use_state(cx, || String::from("Text Button"));

    render!(
        Look {
            name: "TextButton",
            controls: render!(
                TextField { label : "Label", value : label, onchange : move | event : FormEvent |
                label.set(event.data.value.clone()) }
            ),
            Button { onclick: |_| {}, &*** label }
        }
    )
}

fn app(cx: Scope) -> Element {
    register("Button", ButtonPage);
    register("TextButton", TextButtonPage);

    render!(LookBook {})
}

fn main() {
    dioxus_web::launch(app);
}

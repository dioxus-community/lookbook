use dioxus::prelude::*;
use dioxus_material::{Button, TextButton, TextField};
use lookbook::{Look, LookBook};
use lookbook_macros::preview;

#[preview]
fn ButtonPreview(cx: Scope) -> Element {
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

#[preview]
fn TextButtonPreview(cx: Scope) -> Element {
    let label = use_state(cx, || String::from("Text Button"));

    render!(
        Look {
            name: "TextButton",
            controls: render!(
                TextField { label : "Label", value : label, onchange : move | event : FormEvent |
                label.set(event.data.value.clone()) }
            ),
            TextButton { onclick: |_| {}, &*** label }
        }
    )
}

fn app(cx: Scope) -> Element {
    render!(LookBook {
        previews: [ButtonPreview, TextButtonPreview]
    })
}

fn main() {
    dioxus_web::launch(app);
}

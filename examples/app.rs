use dioxus::prelude::*;
use dioxus_material::{Button, TextButton, TextField};
use lookbook::{Look, LookBook};
use lookbook_macros::preview;

#[preview]
fn ButtonPreview(cx: Scope) -> Element {
    let label = use_state(cx, || String::from("Filled Button"));

    let controls = render!(TextField {
        label: "Label",
        value: label,
        onchange: move |event: FormEvent| label.set(event.data.value.clone())
    });

    render!(
        Look { name: "Button", controls: controls,
            Button { onpress: |_| {}, &*** label }
        }
    )
}

#[preview]
fn TextButtonPreview(cx: Scope) -> Element {
    let label = use_state(cx, || String::from("Text Button"));

    let controls = render!(TextField {
        label: "Label",
        value: label,
        onchange: move |event: FormEvent| label.set(event.data.value.clone())
    });

    render!(
        Look { name: "TextButton", controls: controls,
            TextButton { onpress: |_| {}, &*** label }
        }
    )
}

#[component]
fn Home(cx: Scope) -> Element {
    render!( h1 { "Lookbook Example" } )
}

fn app(cx: Scope) -> Element {
    render!(LookBook {
        home: Home,
        previews: [ButtonPreview, TextButtonPreview]
    })
}

fn main() {
    dioxus_web::launch(app);
}

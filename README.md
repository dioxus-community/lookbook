# Lookbook
UI preview framework for Dioxus

```rust
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
            Button { onclick: |_| {}, &*** label }
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
        previews: [ButtonPreview]
    })
}
```
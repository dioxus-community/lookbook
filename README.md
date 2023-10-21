# Lookbook

```rust
#[component]
fn ButtonPage(cx: Scope) -> Element {
    let label = use_state(cx, || String::from("Filled Button"));

    render!(
        Look {
            name: "Button",
            controls: render!(
                TextField { label: "Label", value: label, onchange: move | event: FormEvent |
                label.set(event.data.value.clone()) }
            ),
            Button { onclick: |_| {}, &*** label }
        }
    )
}

fn app(cx: Scope) -> Element {
    register("Button", ButtonPage);
   
    render!(LookBook {})
}
```
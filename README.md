# Lookbook
UI preview framework for Dioxus

```rust
/// To-Do Task.
#[preview]
pub fn TaskPreview<'a>(
    cx: Scope<'a>,

    /// Label of the task.
    #[lookbook(default = "Ice skating")]
    label: &'a str,

    /// Content of the task.
    #[lookbook(default = "Central Park")]
    content: &'a str,
) -> Element<'a> {
    render!(
        div {
            h4 { "{label}" }
            p { "{content}" }
        }
    )
}
```

## Usage
First add Lookbook as a dependency to your project.

```sh
cargo add lookbook --git https://github.com/matthunz/lookbook
```

Then create a preview like the one above and add it to a lookbook.

```rust
fn app(cx: Scope) -> Element {
    render!(LookBook {
        home: |cx| render!("Home"),
        previews: [TaskPreview]
    })
}

fn main() {
    dioxus_web::launch(app)
}
```
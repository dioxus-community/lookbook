<div align="center">
<h1>Lookbook</h1>
<h4>UI preview framework for Dioxus<h4>
<a href="https://crates.io/crates/lookbook">
    <img src="https://img.shields.io/crates/v/lookbook?style=flat-square"alt="Crates.io version" />
</a>
<a href="https://docs.rs/lookbook/latest/lookbook">
    <img src="https://img.shields.io/badge/docs-stable-blue.svg?style=flat-square"alt="docs.rs docs" />
</a>
<a href="https://dioxus-material.netlify.app/dioxus_material/">
    <img src="https://img.shields.io/badge/docs-nightly-blue.svg?style=flat-square"
  alt="nightly docs" />
</a>
<a href="https://github.com/matthunz/dioxus-material/actions">
    <img src="https://github.com/matthunz/dioxus-material/actions/workflows/ci.yml/badge.svg"
  alt="CI status" />
</a>
</div>

<br>

[Demo](https://matthunz.github.io/)

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

Run with `dx serve`!

## Running examples
Run the examples with `dx serve --example {name}`.

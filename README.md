<div align="center">
<h1>Lookbook</h1>
<h4>UI preview framework for Dioxus<h4>
<a href="https://crates.io/crates/lookbook">
    <img src="https://img.shields.io/crates/v/lookbook?style=flat-square"alt="Crates.io version" />
</a>
<a href="https://docs.rs/lookbook/0.2.0-alpha.1/lookbook/index.html">
    <img src="https://img.shields.io/badge/docs-stable-blue.svg?style=flat-square"alt="docs.rs docs" />
</a>
<a href="https://github.com/matthunz/dioxus-material/actions">
    <img src="https://github.com/matthunz/lookbook/actions/workflows/ci.yml/badge.svg"
  alt="CI status" />
</div>
<div align="center">
    <a href="https://dioxus-material-lookbook.netlify.app/">Demo</a>
</div>

<br>

```rs
/// To-Do Task.
#[preview]
pub fn TaskPreview(
    /// Label of the task.
    #[lookbook(default = "Ice skating")]
    label: String,

    /// Content of the task.
    #[lookbook(default = "Central Park")]
    content: String,

    /// List of tags.
    #[lookbook(default = vec![String::from("A")])]
    tags: Json<Vec<String>>,
) -> Element {
    rsx!(
        div {
            h4 { "{label}" }
            p { "{content}" }
            div { { tags.0.iter().map(|tag| rsx!(li { "{tag}" })) } }
        }
    )
}

#[component]
fn app() -> Element {
    rsx!(LookBook {
        home: |()| rsx!("Home"),
        previews: [TaskPreview]
    })
}

fn main() {
    dioxus::launch(app)
}
```

## Usage
First add Lookbook as a dependency to your project.

```sh
cargo add lookbook@v0.2.0-alpha.1
```

Then create a preview like the one above and add it to a lookbook.

```rust
fn app() -> Element {
    rsx!(LookBook {
        home: |()| rsx!("Home"),
        previews: [TaskPreview]
    })
}

fn main() {
    dioxus::launch(app)
}
```

Run with `dx serve`!

## Supported platforms
 - [x] Web
 - [ ] Desktop/mobile https://github.com/matthunz/lookbook/issues/5
 - [ ] Blitz https://github.com/matthunz/lookbook/issues/6

## Running examples
Run the examples with `dx serve --example {name}`.

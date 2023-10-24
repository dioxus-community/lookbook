use dioxus::prelude::*;
use lookbook::LookBook;
use lookbook_macros::preview;

/// To-Do Task.
#[preview]
pub fn TaskPreview<'a>(
    cx: Scope<'a>,

    /// Label of the task.
    label: &'a str,

    /// Content of the task.
    content: &'a str,
) -> Element<'a> {
    render!(
        div {
            h4 { "{label}" }
            p { "{content}" }
        }
    )
}

fn app(cx: Scope) -> Element {
    render!(LookBook {
        home: |cx| render!("Home"),
        previews: [TaskPreview]
    })
}

fn main() {
    dioxus_web::launch(app)
}

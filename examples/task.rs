use dioxus::prelude::*;
use lookbook::LookBook;
use lookbook_macros::preview;

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

fn app(cx: Scope) -> Element {
    render!(LookBook {
        home: |cx| render!("Home"),
        previews: [TaskPreview]
    })
}

fn main() {
    dioxus_web::launch(app)
}

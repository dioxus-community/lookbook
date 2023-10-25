use dioxus::prelude::*;
use lookbook::{Json, LookBook};
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

    /// List of tags.
    #[lookbook(default = vec![String::from("A")])]
    tags: Json<Vec<String>>,
) -> Element<'a> {
    render!(
        div {
            h4 { "{label}" }
            p { "{content}" }
            div { tags.0.iter().map(|tag| render!(li { "{tag}" })) }
        }
    )
}

fn app(cx: Scope) -> Element {
    render!( LookBook { home: |cx| render!("Home"), previews: [TaskPreview] } )
}

fn main() {
    dioxus_web::launch(app)
}

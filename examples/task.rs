use dioxus::prelude::*;
use lookbook::LookBook;
use lookbook_macros::preview;

/// Task docs
///
/// ## Props
/// - label: &str
/// - content: &str
#[preview]
pub fn TaskPreview<'a>(cx: Scope<'a>, label: &'a str, content: &'a str) -> Element<'a> {
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

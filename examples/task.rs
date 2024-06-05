use dioxus::prelude::*;
use lookbook::{Json, LookBook};
use lookbook_macros::preview;


/// To-Do Task.
#[preview]
pub fn TaskPreview(
    /// Label of the task.
    #[lookbook(default = "Ice skating")]
    label: &'a str,

    /// Content of the task.
    #[lookbook(default = "Central Park")]
    content: &'a str,

    /// List of tags.
    #[lookbook(default = vec![String::from("A")])]
    tags: Json<Vec<String>>,
) -> Element {
    rsx!(
        div {
            h4 { "{label}" }
            p { "{content}" }
            div { tags.0.iter().map(|tag| rsx!(li { "{tag}" })) }
        }
    )
}

fn app() -> Element {
    rsx!(LookBook {
        home: |cx| rsx!("Home"),
        previews: []
    })
}


fn main() {
    dioxus_web::launch::launch_cfg(app, Default::default())
}

use dioxus::prelude::*;
use pulldown_cmark::Parser;

/// Render some text as markdown.
#[component]
pub fn Markdown(
    #[props(default)] id: Signal<String>,
    #[props(default)] class: Signal<String>,

    content: ReadOnlySignal<String>,
) -> Element {
    let content = &*content.read();
    let parser = Parser::new(content);

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    rsx! {
        div {
            id: "{&*id.read()}",
            class: "{&*class.read()}",
            dangerous_inner_html: "{html_buf}"
        }
    }
}

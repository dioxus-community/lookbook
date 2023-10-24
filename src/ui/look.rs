use crate::ui::pane::VerticalPane;
use dioxus::prelude::*;
use dioxus_markdown::Markdown;

#[component]
pub fn Look<'a>(
    cx: Scope<'a>,
    name: &'static str,
    docs: &'static str,
    controls: Element<'a>,
    children: Element<'a>,
) -> Element<'a> {
    let top = render!(
        div { padding: "20px",
            h2 { "{name}" }
            Markdown { content: "{docs}" }
        }
        div {
            flex: 1,
            display: "flex",
            flex_direction: "column",
            padding: "20px",
            display: "flex",
            justify_content: "center",
            align_items: "center",
            div { flex: 1, children }
        }
    );

    let bottom = render!(
        div {
            flex: 1,
            display: "flex",
            flex_direction: "column",
            overflow_y: "auto",
            padding: "20px",
            gap: "20px",
            background: "#f9f9f9",
            h4 { "Controls" }
            div { flex: 1, display: "flex", flex_direction: "row", gap: "40px", controls }
        }
    );

    render!(
        div { flex: 1, display: "flex", flex_direction: "column", VerticalPane { top: top, bottom: bottom } }
    )
}

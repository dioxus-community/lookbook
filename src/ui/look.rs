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
            display: "flex",
            justify_content: "center",
            align_items: "center",
            children
        }
    );

    let bottom = render!(
        div { flex: 1, display: "flex", flex_direction: "column", overflow_y: "auto", gap: "20px",
            table { text_align: "left", border_collapse: "collapse",
                tr { height: "60px", color: "#777", border_bottom: "2px solid #e7e7e7",
                    th { padding_left: "20px", "Name" }
                    th { "Type" }
                    th { "Description" }
                    th { "Default" }
                    th { padding_right: "20px", "Controls" }
                }
                controls
            }
        }
    );

    render!(
        div { flex: 1, display: "flex", flex_direction: "column", VerticalPane { top: top, bottom: bottom } }
    )
}

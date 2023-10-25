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
        div { flex: 1, display: "flex", flex_direction: "column", overflow_y: "auto", padding: "20px", gap: "20px",
            table { text_align: "left", border_collapse: "collapse",
                tr { border_bottom: "2px solid #e7e7e7",
                    Th { "Name" }
                    Th { "Description" }
                    Th { "Default" }
                    Th { "Controls" }
                }
                controls
            }
        }
    );

    render!(
        div { flex: 1, display: "flex", flex_direction: "column", VerticalPane { top: top, bottom: bottom } }
    )
}

#[component]
pub fn Th<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    render!(th {
        padding_bottom: "10px",
        children
    })
}

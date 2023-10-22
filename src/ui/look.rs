use crate::ui::pane::VerticalPane;
use dioxus::prelude::*;

#[component]
pub fn Look<'a>(
    cx: Scope<'a>,
    name: &'static str,
    controls: Element<'a>,
    children: Element<'a>,
) -> Element<'a> {
    render!(
        div { flex: 1, display: "flex", flex_direction: "column",
            VerticalPane {
                top: render!(
                    div { padding : "20px", h2 { "{name}" } } div { flex : 1, display : "flex",
                    flex_direction : "column", padding : "20px", display : "flex", justify_content :
                    "center", align_items : "center", div { flex : 1, children } }
                ),
                bottom: render!(
                    div { flex : 1, overflow_y : "auto", padding : "20px", background : "#f9f9f9", h4 {
                    "Controls" } controls }
                )
            }
        }
    )
}

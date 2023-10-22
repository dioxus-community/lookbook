use dioxus::prelude::*;

#[component]
pub fn HorizontalPane<'a>(cx: Scope<'a>, left: Element<'a>, right: Element<'a>) -> Element<'a> {
    let width = use_state(cx, || 200.);
    let is_dragging = use_state(cx, || false);

    render!(
        div {
            position: "relative",
            flex: 1,
            display: "flex",
            flex_direction: "row",
            onmouseup: move |_| { is_dragging.set(false) },
            onmousemove: move |event| {
                if **is_dragging {
                    width.set(event.data.client_coordinates().x)
                }
            },
            div { display: "flex", flex_direction: "row", width: "{width}px", overflow: "auto",
                left,
                div {
                    height: "100%",
                    padding: "0 5px",
                    margin: "0 -5px",
                    cursor: "ew-resize",
                    onmousedown: move |_| { is_dragging.set(true) },
                    div { width: "2px", height: "100%", background: "#ccc" }
                }
            }
            right
        }
    )
}

#[component]
pub fn VerticalPane<'a>(cx: Scope<'a>, top: Element<'a>, bottom: Element<'a>) -> Element<'a> {
    let height = use_state(cx, || 200.);
    let is_dragging = use_state(cx, || false);

    render!(
        div { position: "relative", flex: 1, display: "flex", flex_direction: "column",
            div {
                position: "absolute",
                display: if **is_dragging { "block" } else { "none" },
                width: "100%",
                height: "100%",
                onmouseup: move |_| { is_dragging.set(false) },
                onmousemove: move |event| height.set(event.data.client_coordinates().y)
            }
            div { display: "flex", flex_direction: "column", height: "{height}px", overflow: "auto", top }
            div {
                width: "100%",
                padding: "5px 0",
                margin: "-5px 0",
                cursor: "ns-resize",
                onmousedown: move |_| { is_dragging.set(true) },
                div { height: "2px", width: "100%", background: "#ccc" }
            }
            bottom
        }
    )
}

use dioxus::prelude::*;

fn app(cx: Scope) -> Element {
    render!(div {})
}

fn main() {
    dioxus_web::launch(app);
}

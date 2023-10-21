use dioxus::prelude::*;
use lookbook::{register, LookBook};

fn app(cx: Scope) -> Element {
    register(
        String::from("Button"),
        |cx| render!( div { "This is a filled button!" } ),
    );

    register(
        String::from("Text Button"),
        |cx| render!( div { "This is a text button!" } ),
    );

    render!( LookBook {} )
}

fn main() {
    dioxus_web::launch(app);
}

use wasm_bindgen::JsCast;
use xilem_web::{
    elements::html,
    interfaces::{Element, HtmlElement},
    style, BoxedView, View, ViewMarker,
};

pub trait Properties: Sized {
    fn properties(self) -> impl View<Self> + ViewMarker + 'static;
}

pub trait Property: Sized {
    fn property(
        self,
        name: &'static str,
        default: Self,
        description: Option<&'static str>,
    ) -> impl View<Self> + 'static;
}

impl Property for String {
    fn property(
        self,
        name: &'static str,
        default: Self,
        description: Option<&'static str>,
    ) -> impl View<Self> + 'static {
        html::li((
            html::span(name).style(style("width", "25%")),
            html::span(default).style(style("width", "25%")),
            html::span(description).style(style("width", "25%")),
            html::input(())
                .attr("value", self)
                .style(style("width", "25%"))
                .on_input(|me, event| {
                    if let Some(element) = event
                        .target()
                        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                    {
                        *me = element.value();
                        event.prevent_default();
                    }
                })
                .passive(false),
        ))
        .style(style("flex-direction", "row"))
    }
}

use lookbook::{Properties, Property};
use xilem_web::{elements::html, interfaces::HtmlElement, style, View, ViewExt, ViewMarker};

#[derive(Clone)]
struct ButtonPreview {
    label: String,
}

impl Properties for ButtonPreview {
    fn properties(self) -> impl View<Self> + ViewMarker + 'static {
        self.label
            .property("Label", String::from("Example"), None)
            .adapt_state(|me: &mut Self| &mut me.label)
    }
}

fn button_preview(state: &ButtonPreview) -> impl View<ButtonPreview> + ViewMarker {
    html::button(state.label.clone())
}

fn app(state: &mut ButtonPreview) -> impl View<ButtonPreview> {
    html::div((state.clone().properties(), button_preview(&state)))
        .style((style("display", "flex"), style("flex-direction", "row")))
}

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    xilem_web::App::new(
        ButtonPreview {
            label: String::from("Example"),
        },
        app,
    )
    .run(&xilem_web::document_body());
}

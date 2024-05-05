use xilem_web::{
    elements::html,
    interfaces::{Element, HtmlElement},
    style, Action, MessageResult, View, ViewExt,
};

pub enum SidebarAction {
    Select(usize),
}

impl Action for SidebarAction {}

pub fn sidebar(selected: usize, items: &[&'static str]) -> impl View<(), SidebarAction> {
    html::ul(
        items
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                html::li(*item)
                    .style((idx == selected).then(|| style("background", "purple")))
                    .on_click(move |_, _| SidebarAction::Select(idx))
            })
            .collect::<Vec<_>>(),
    )
}

pub struct Lookbook {
    selected: usize,
    previews: Vec<&'static str>,
}

fn app(state: &mut Lookbook) -> impl View<Lookbook> {
    sidebar(state.selected, &state.previews).adapt(|state: &mut Lookbook, thunk| {
        if let MessageResult::Action(action) = thunk.call(&mut ()) {
            match action {
                SidebarAction::Select(idx) => state.selected = idx,
            }
        }
        xilem_web::MessageResult::Nop
    })
}

fn main() {
    xilem_web::App::new(
        Lookbook {
            selected: 0,
            previews: vec!["Filled button", "Outlined button"],
        },
        app,
    )
    .run(&xilem_web::document_body());
}

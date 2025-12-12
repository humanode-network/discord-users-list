use iced::{
    Element,
    task::Handle,
    widget::{button, column, text},
};

#[derive(Debug)]
pub struct State {
    pub abort: Handle,
    pub configured_params: crate::logic::export_members::Params,
}

#[derive(Debug, Clone)]
pub enum Message {
    Cancel,
}

#[derive(Debug, Clone)]
pub enum Action {
    CancelExport(crate::logic::export_members::Params),
}

pub fn update(state: &mut State, message: Message) -> Option<Action> {
    match message {
        Message::Cancel => Some(Action::CancelExport(state.configured_params.clone())),
    }
}

pub fn view(_state: &State) -> Element<'_, Message> {
    column![
        text("Exporting members"),
        button("Cancel").on_press(Message::Cancel)
    ]
    .spacing(20)
    .into()
}

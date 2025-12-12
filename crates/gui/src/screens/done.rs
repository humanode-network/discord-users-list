use iced::{
    Element,
    widget::{button, column, text},
};

#[derive(Debug)]
pub struct State {
    pub error: Option<String>,
    pub configured_params: crate::logic::export_members::Params,
}

#[derive(Debug, Clone)]
pub enum Message {
    Again,
}

#[derive(Debug, Clone)]
pub enum Action {
    Again(crate::logic::export_members::Params),
}

pub fn update(state: &mut State, message: Message) -> Option<Action> {
    match message {
        Message::Again => Some(Action::Again(state.configured_params.clone())),
    }
}

pub fn view(state: &State) -> Element<'_, Message> {
    column![
        text(if state.error.is_some() {
            "Error"
        } else {
            "Done"
        }),
        state.error.as_ref().map(text),
        button("Again").on_press(Message::Again),
    ]
    .spacing(20)
    .into()
}

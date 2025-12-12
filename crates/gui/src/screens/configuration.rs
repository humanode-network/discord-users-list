use iced::{
    Element,
    widget::{button, column, text, text_input},
};

#[derive(Debug)]
pub struct State {
    pub params: crate::logic::export_members::Params,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetDiscordToken(String),
    SetDiscordServerId(String),
    SetOutput(String),
    StartMembersExport,
}

#[derive(Debug, Clone)]
pub enum Action {
    StartMembersExport(crate::logic::export_members::Params),
}

pub fn update(state: &mut State, message: Message) -> Option<Action> {
    match message {
        Message::SetDiscordToken(val) => {
            state.params.discord_token = val;
            None
        }
        Message::SetDiscordServerId(val) => {
            state.params.discord_server_id = val;
            None
        }
        Message::SetOutput(val) => {
            state.params.output = val;
            None
        }
        Message::StartMembersExport => Some(Action::StartMembersExport(state.params.clone())),
    }
}

pub fn view(state: &State) -> Element<'_, Message> {
    column![
        text_input("Discord Token", &state.params.discord_token).on_input(Message::SetDiscordToken),
        text_input("Discord Server ID", &state.params.discord_server_id)
            .on_input(Message::SetDiscordServerId),
        text_input("Output File", &state.params.output).on_input(Message::SetOutput),
        button(text("Export members")).on_press(Message::StartMembersExport)
    ]
    .spacing(20)
    .into()
}

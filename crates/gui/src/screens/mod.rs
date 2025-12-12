#![allow(missing_docs, clippy::missing_docs_in_private_items)]

use iced::Element;

pub mod configuration;
pub mod done;
pub mod exporting;

pub enum State {
    Configuration(configuration::State),
    Exporting(exporting::State),
    Done(done::State),
}

#[derive(Debug, Clone)]
pub enum Message {
    Configuration(configuration::Message),
    Exporting(exporting::Message),
    Done(done::Message),
}

pub fn view(state: &State) -> Element<'_, Message> {
    match state {
        State::Configuration(state) => configuration::view(state).map(Message::Configuration),
        State::Exporting(state) => exporting::view(state).map(Message::Exporting),
        State::Done(state) => done::view(state).map(Message::Done),
    }
}

pub enum Action {
    Configuration(configuration::Action),
    Exporting(exporting::Action),
    Done(done::Action),
}

pub fn update(state: &mut State, message: Message) -> Option<Action> {
    match (state, message) {
        (State::Configuration(state), Message::Configuration(message)) => {
            configuration::update(state, message).map(Action::Configuration)
        }
        (State::Exporting(state), Message::Exporting(message)) => {
            exporting::update(state, message).map(Action::Exporting)
        }
        (State::Done(state), Message::Done(message)) => {
            done::update(state, message).map(Action::Done)
        }
        _ => unreachable!(),
    }
}

//! The UI.

use iced::{Element, Task, widget::container};

use crate::screens;

/// The overall UI state.
pub type State = screens::State;

/// The global UI messages.
#[derive(Debug, Clone)]
pub enum Message {
    /// Screens' own messages.
    Screens(screens::Message),

    /// Export operation has completed.
    ExportDone {
        /// Export error, if any.
        error: Option<String>,

        /// The params used to run the operation.
        params: crate::logic::export_members::Params,
    },
}

/// Initial screens state.
pub fn boot() -> State {
    let discord_token = envfury::must("DISCORD_TOKEN").unwrap_or_default();
    let discord_server_id = envfury::must("DISCORD_SERVER_ID").unwrap_or_default();
    let output = envfury::must("OUTPUT").unwrap_or_default();

    let params = crate::logic::export_members::Params {
        discord_token,
        discord_server_id,
        output,
    };

    screens::State::Configuration(screens::configuration::State { params })
}

/// The global UI view fn.
pub fn view(state: &State) -> Element<'_, Message> {
    container(screens::view(state).map(Message::Screens))
        .padding(20)
        .into()
}

/// The global UI update fn.
pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Screens(message) => {
            let Some(action) = screens::update(state, message) else {
                return Task::none();
            };

            match action {
                screens::Action::Configuration(
                    screens::configuration::Action::StartMembersExport(params),
                ) => {
                    let (task, abort) =
                        Task::perform(crate::logic::export_members::run(params.clone()), {
                            let params = params.clone();
                            move |result| {
                                let error = result.err().map(|err| err.to_string());
                                Message::ExportDone { error, params }
                            }
                        })
                        .abortable();

                    *state = State::Exporting(screens::exporting::State {
                        abort,
                        configured_params: params,
                    });

                    task
                }
                screens::Action::Exporting(screens::exporting::Action::CancelExport(params)) => {
                    let old_state = core::mem::replace(
                        state,
                        State::Configuration(screens::configuration::State { params }),
                    );

                    if let screens::State::Exporting(state) = old_state {
                        state.abort.abort()
                    }

                    Task::none()
                }
                screens::Action::Done(screens::done::Action::Again(params)) => {
                    *state = State::Configuration(screens::configuration::State { params });
                    Task::none()
                }
            }
        }
        Message::ExportDone { error, params } => {
            *state = screens::State::Done(screens::done::State {
                error,
                configured_params: params,
            });
            Task::none()
        }
    }
}

//! List discord server members and write the output to a file
//! in the `JSONLines` format.

mod logic {
    //! The logic of the operations.

    pub mod export_members;
}
mod screens;
mod ui;

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(ui::boot, ui::update, ui::view)
        .executor::<tokio::runtime::Runtime>()
        .run()
}

mod notes_application;

fn main() -> iced::Result {
    iced::run("notesappv2-0.01", notes_application::App::update, notes_application::App::view)
}




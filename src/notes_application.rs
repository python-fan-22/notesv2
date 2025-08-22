#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed,
}

pub enum PauseStart {
    Pause(String),
    Start(String),
}

#[derive(Default)]
pub struct App {
    counter: usize,
}

struct Metronome {
    note: String,
    pause_start_button: PauseStart,
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => self.counter += 1,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        iced::widget::column![
            iced::widget::text(self.counter),
            iced::widget::button("Increase").on_press(Message::ButtonPressed),
        ]
            .into()
    }
}
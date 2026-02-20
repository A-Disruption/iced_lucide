mod icon;

use iced::widget::{row};
use iced::Element;

fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .title("Lucide Icons")
        .font(icon::FONT)
        .run()
}

#[derive(Clone, Debug)]
enum Message {
    IconSelected(String),
}

#[derive(Default)]
struct Example;

impl Example {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::IconSelected(name) => {
                println!("{} selected!", name);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        row![
            icon::edit(),
            icon::save(),
            icon::trash(),
            icon::search(),
            icon::home(),
            icon::settings(),
        ]
        .spacing(8)
        .into()
    }
}

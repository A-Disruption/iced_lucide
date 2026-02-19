mod icon;

use iced::widget::{button, container, scrollable, text, tooltip, Row};
use iced::Element;

use crate::icon::ALL_ICONS;

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
        let icon_buttons: Vec<Element<'_, Message>> = ALL_ICONS
            .iter()
            .map(|(name, codepoint)| {
                tooltip(
                    button(icon::render(codepoint))
                        .on_press(Message::IconSelected(name.to_string())),
                    container(text(*name)).style(container::bordered_box).padding(5),
                    tooltip::Position::Top,
                )
                .into()
            })
            .collect();

        scrollable(
            Row::with_children(icon_buttons)
                .spacing(4)
                .padding(10)
                .wrap(),
        )
        .into()
    }
}

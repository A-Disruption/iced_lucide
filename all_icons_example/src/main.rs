mod icon;

use iced::Element;
use iced::widget::{Row, button, container, scrollable, text, tooltip};

use crate::icon::ALL_ICONS;

fn main() -> iced::Result {
    let mut app =
        iced::application(Example::default, Example::update, Example::view).title("Lucide Icons");

    for font in icon::FONTS {
        app = app.font(*font);
    }

    app.run()
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
                println!("{name} selected!");
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let icon_buttons: Vec<Element<'_, Message>> = ALL_ICONS
            .iter()
            .map(|icon| {
                tooltip(
                    button(icon::render(*icon))
                        .on_press(Message::IconSelected(icon.name.to_string())),
                    container(text(icon.name))
                        .style(container::bordered_box)
                        .padding(5),
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

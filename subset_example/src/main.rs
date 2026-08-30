mod icon;

use iced::Element;
use iced::widget::{button, column, row, text};

fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title("iced_lucide — mixed icon sets");

    // Each family contributes its own subset font, so register them all.
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
        let lucide = row![
            icon::edit(),
            icon::save(),
            icon::trash(),
            icon::search(),
            icon::home(),
            icon::settings(),
        ]
        .spacing(8);

        let others = row![
            icon::github(),
            icon::heart(),
            icon::bluetooth(),
            icon::wifi(),
            icon::debug(),
            icon::terminal(),
        ]
        .spacing(8);

        // ALL_ICONS carries the font alongside each codepoint, so a picker can
        // render icons from several families without tracking which is which.
        let every = row(icon::ALL_ICONS.iter().map(|icon| {
            button(icon::render(*icon))
                .on_press(Message::IconSelected(icon.name.to_string()))
                .into()
        }))
        .spacing(8);

        column![
            text("Lucide"),
            lucide,
            text("Font Awesome, Bootstrap, Codicons"),
            others,
            text("Everything in this module, via ALL_ICONS"),
            every,
        ]
        .spacing(12)
        .padding(20)
        .into()
    }
}

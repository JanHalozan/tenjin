use iced::{alignment::Horizontal, widget::{button, container, row, Container}, Alignment, Border, Element, Length, Theme};

use crate::model::{app_message::AppMessage, intent::Intent};

pub struct IntentSuggestion {
    intent: Intent,
    display_text: String
}

impl IntentSuggestion {
    pub fn new(title: String) -> Self {
        Self {
            intent: Intent::default(),
            display_text: title
        }
    }

    pub fn view(&self, _i: usize, highlighted: bool) -> Element<AppMessage> {

        let styler: fn(&Theme) -> container::Style = match highlighted {
            true => |t| container::Style {
                border: Border {
                    color: t.palette().primary.scale_alpha(0.25),
                    radius: 5.0.into(),
                    width: 1.0
                },
                background: Some(iced::Background::Color(t.palette().primary.scale_alpha(0.05))),
                ..Default::default()
            },
            false => |_| Default::default()
        };

        row![
            Container::new(
                button(self.display_text.as_str())
                    .on_press(AppMessage::IntentSubmit)
                    .padding(10)
                    .style(button::text)
            )
            .center_x(Length::Fill)
            .align_x(Horizontal::Left)
            .style(styler)
        ]
        .spacing(20)
        .align_y(Alignment::Center)
        .into()
    }
}
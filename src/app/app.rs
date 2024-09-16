use iced::widget::{text_input, Column, Container, TextInput};
use iced::{window, Alignment, Element, Length, Size, Subscription, Task};

use super::app_message::AppMessage;
use super::subscriptions::app_subscriptions;

pub struct App {
    window_id: Option<window::Id>,
    command_field_id: text_input::Id,
    current_input: String
}

impl App {
    // pub fn new() -> (Self, Task<AppMessage>) {
    //     // let (id, open) = window::open(window::Settings::default());

    //     let app = App {
    //         window_id: None,
    //         current_input: String::new()
    //     };

    //     (app, Task::none())
    //     // (app, open.map(AppMessage::WindowOpened))
    // }

    pub fn title(&self, _window: window::Id) -> String {
        "Tenjin".to_string()
    }

    pub fn view(&self, _window_id: window::Id) -> Element<AppMessage> {
        let command_box = TextInput::new("Tenjin command", &self.current_input)
            .id(self.command_field_id.clone())
            .padding(15)
            .size(30)
            .on_input(AppMessage::CommandInputChanged)
            .on_submit(AppMessage::CommandSubmit);

        Container::new(
            Column::new()
                .push(command_box)
                .align_x(Alignment::Center)
                .spacing(20)
        )
        .center_x(0)
        .center_y(0)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        println!("App message {:?}", message);
        match message {
            AppMessage::CommandInputChanged(input) => { 
                self.current_input = input;
            },
            AppMessage::CommandSubmit => {

            },
            AppMessage::WindowOpened(_id) => {
                return text_input::focus(self.command_field_id.clone()).into();
                // let size = Size::<f32> { width: 600.0, height: 80.0 };
                // return window::resize(id, size);
            },
            AppMessage::WindowClosed(_id) => {
                self.window_id = None;
            },
            AppMessage::ToggleVisibility => {
                if let Some(id) = self.window_id {
                    return window::close(id).into();
                } else {
                    let settings = window::Settings {
                        size: Size { width: 600.0, height: 80.0 },
                        decorations: false,
                        transparent: true,
                        level: window::Level::AlwaysOnTop,
                        ..Default::default()
                    };

                    let (id, task) = window::open(settings);
                    self.window_id = Some(id);

                    return task.map(AppMessage::WindowOpened);
                }
            }
        }

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<AppMessage> {
        Subscription::batch(app_subscriptions())
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            window_id: None,
            command_field_id: text_input::Id::unique(),
            current_input: String::new()
        }
    }
}
use std::cmp::{min, max};

use iced::widget::{column, container, text_input, TextInput};
use iced::{window, Element, Length, Size, Subscription, Task};

use crate::model::app_message::{AppMessage, Direction};
use super::components::intent_suggestion::IntentSuggestion;
use super::subscriptions::app_subscriptions;

pub struct App {
    open_window_on_start: bool,

    window_id: Option<window::Id>,
    command_field_id: text_input::Id,
    current_input: String,

    suggestions: Vec<IntentSuggestion>,
    highlighted_suggestion: Option<i32>
}

impl App {
    pub fn new() -> (Self, Task<AppMessage>) {
        let mut app = App::default();

        if app.open_window_on_start {
            let (id, open) = window::open(default_window_settings());
            app.window_id = Some(id);

            (app, open.map(AppMessage::WindowOpened))
        } else {
            (app, Task::none())
        }
    }

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

        let suggestions = column(
            self.suggestions.iter()
            .enumerate()
            .map(|(index, suggestion)| {
                let highlighted = match self.highlighted_suggestion {
                    Some(val) => val as usize == index,
                    _ => false
                };

                suggestion.view(index, highlighted)
            })
        );
        
        container(
            column![command_box, suggestions]
                .spacing(5)
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
            AppMessage::SuggestionMove(dir) => {
                let index = match self.highlighted_suggestion {
                    Some(val) => {
                        let desired_index: i32 = val + dir.increment();
                        min(max(0, desired_index), (self.suggestions.len() as i32) - 1)
                    }
                    None => {
                        if dir == Direction::Up {
                            (self.suggestions.len() as i32) - 1
                        } else {
                            0
                        }
                    }
                };

                self.highlighted_suggestion = Some(index);
                return Task::done(AppMessage::IntentSubmit);
            },
            AppMessage::IntentSubmit => {
                
            },
            AppMessage::CommandInputChanged(input) => { 
                self.current_input = input;
                self.highlighted_suggestion = None;
                return Task::done(AppMessage::CommandSubmit);
            },
            AppMessage::CommandSubmit => {
                self.suggestions = self.intent_suggestions(&self.current_input);

                let items_height = self.suggestions.len() as f32 * 50.0;
                let size = Size::new(800.0, 60.0 + items_height);
                return window::resize(self.window_id.unwrap(), size);
            },
            AppMessage::WindowOpened(id) => {
                let input_focus_task = text_input::focus(self.command_field_id.clone());

                return window::gain_focus(id)
                    .chain(input_focus_task);
            },
            AppMessage::WindowClosed(_id) => {
                self.window_id = None;
            },
            AppMessage::ToggleVisibility => {
                if let Some(id) = self.window_id {
                    return window::close(id).into();
                } else {
                    let settings = default_window_settings();
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

    fn intent_suggestions(&self, needle: &str) -> Vec<IntentSuggestion> {
        let options = vec![
            IntentSuggestion::new("Enable weird symbols".to_string()),
            IntentSuggestion::new("Search ChatGPT".to_string()),
            IntentSuggestion::new("Do the derp".to_string()),
            IntentSuggestion::new("Insert smart snippet".to_string()),
            IntentSuggestion::new("Begin drone mission".to_string()),
            IntentSuggestion::new("Create a new extension".to_string())
        ];

        use std::time::{SystemTime, UNIX_EPOCH};
        let time_seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
        let count = (time_seed % 7 + 2) as usize; // Get a random number between 2 and 8
        options.into_iter().take(count).collect()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            open_window_on_start: true,
            window_id: None,
            command_field_id: text_input::Id::unique(),
            current_input: String::new(),
            suggestions: vec![],
            highlighted_suggestion: None
        }
    }
}

fn default_window_settings() -> window::Settings {
    window::Settings {
        size: Size { width: 600.0, height: 800.0 },
        decorations: false,
        transparent: true,
        level: window::Level::AlwaysOnTop,
        ..Default::default()
    }
}
use iced::window;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Up, Down
}

impl Direction {
    pub fn increment(&self) -> i32 {
        if *self == Direction::Up {
            -1
        } else {
            1
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    ToggleVisibility, WindowOpened(window::Id), WindowClosed(window::Id),
    
    
    CommandInputChanged(String), CommandSubmit, SuggestionMove(Direction),

    IntentSubmit
}
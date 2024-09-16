use iced::window;

#[derive(Debug, Clone)]
pub enum AppMessage {
    ToggleVisibility, WindowOpened(window::Id), WindowClosed(window::Id),
    
    
    CommandInputChanged(String), CommandSubmit
}
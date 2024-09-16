use iced::{keyboard::{key, on_key_press, Key, Modifiers}, window, Subscription};

use crate::listener::watchdog_listener::create_watchdog_listener;

use super::app_message::AppMessage;

pub fn app_subscriptions() -> Vec<Subscription<AppMessage>> {
    vec![
        Subscription::run(create_watchdog_listener),
        window::close_events().map(AppMessage::WindowClosed),
        on_key_press(handle_keyboard_hotkeys)
    ]
}

fn handle_keyboard_hotkeys(key: Key, _modifiers: Modifiers) -> Option<AppMessage> {
    match key.as_ref() {
        Key::Named(key::Named::Escape) => {
            Some(AppMessage::ToggleVisibility)
        }
        _ => None
    }
}
use std::thread;

use iced::futures::{SinkExt, Stream};
use rdev::{Event, EventType, Key, ListenError};
use tokio::sync::mpsc::Sender as TokioSender;
use iced::futures::channel::mpsc::Sender as IcedSender;

use crate::model::app_message::AppMessage;

pub fn create_watchdog_listener() -> impl Stream<Item = AppMessage> {
    let callback = |mut output: IcedSender<AppMessage>| async move {
        let (sender, mut receiver) = tokio::sync::mpsc::channel(10);

        thread::spawn(move || {
            if let Err(error) = start_listening(sender) {
                eprintln!("rdev event listen failed: {:?}", error);
            }
        });

        while let Some(_) = receiver.recv().await {
            if let Err(error) = output.send(AppMessage::ToggleVisibility).await {
                eprint!("Watchdog channel send error: {:?}", error);
            }
        }
    };

    iced::stream::channel(10, callback)
}

fn start_listening(sender: TokioSender<()>) -> Result<(), ListenError> {
    let mut meta_pressed = false;
    let callback = move |event: Event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                if !handle_key_press(&mut meta_pressed, key) {
                    return;
                }

                if let Err(error) = sender.blocking_send(()) {
                    eprintln!("Watchdog channel send error: {:?}", error);
                }
            },
            EventType::KeyRelease(key) => handle_key_release(&mut meta_pressed, key),
            _ => {}
        }
    };

    rdev::listen(callback)
}

fn handle_key_press(meta_pressed: &mut bool, key: Key) -> bool {
    match key {
        Key::MetaLeft | Key::MetaRight => {
            *meta_pressed = true;
        },
        Key::KeyD => {
            if !*meta_pressed {
                return false;
            }

            return true;
        }
        _ => {}
    }

    false
}

fn handle_key_release(meta_pressed: &mut bool, key: Key) {
    match key {
        Key::MetaLeft | Key::MetaRight => {
            *meta_pressed = false;
        },
        _ => {}
    }
}
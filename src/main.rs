mod app;
mod listener;

use app::app::App;

pub fn main() -> iced::Result {
    iced::daemon(App::title, App::update, App::view)
        .subscription(App::subscription)
        .run()
}

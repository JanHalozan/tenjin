mod app;
mod listener;
mod model;

use app::app::App;

pub fn main() -> iced::Result {
    iced::daemon(App::title, App::update, App::view)
        .subscription(App::subscription)
        .run_with(|| App::new())
}

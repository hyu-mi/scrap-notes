mod app;
mod app_command;
mod app_error;
mod app_event;
mod config;
mod file_manager;
mod metadata;
mod note;

use app::App;
use config::Config;

// Scrap it!
fn main() {
    let config = Config::load();
    let mut app = App::new(config);
    app.run();
}

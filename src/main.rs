mod app;
mod core;

use app::{app::App, config::Config};

// Scrap it!
fn main() {
    let config = Config::load();
    let mut app = App::new(config);
    app.run();
}

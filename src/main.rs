mod core;

use crate::core::app::App;

// Scrap it!
fn main() {
    let path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join(".workspace");
    std::fs::create_dir_all(&path).expect("Failed to create workspace folder");

    let root = path
        .canonicalize()
        .expect("Could not canonicalize root path");

    let app = App::new(root);
    app.run();
}

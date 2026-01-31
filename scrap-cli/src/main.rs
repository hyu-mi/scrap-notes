use clap::Parser;
use std::path::PathBuf;
use uuid::{Uuid, uuid};

mod app;
mod app_error;
mod cli;

use app::App;
use app_error::AppError;
use cli::CliArgs;

fn main() {
    let args = CliArgs::parse();

    let workspace_dir = std::env::current_dir()
        .expect("Failed to get current directory")
        .join(".workspace");

    let mut app = App::new();

    app.init(&workspace_dir).expect("Failed to initialize app");
    app.execute(args.command);
}

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

    let workspace_dir = get_workspace_dir();
    let mut app = App::new(workspace_dir);

    app.init().expect("Failed to initialize app");
    app.execute(args.command);
}

fn get_workspace_dir() -> PathBuf {
    let workspace_dir = std::env::current_dir()
        .expect("Failed to get current directory")
        .join(".workspace");

    std::fs::create_dir_all(&workspace_dir).expect("Failed to create workspace folder");

    let out_path = workspace_dir
        .canonicalize()
        .expect("Failed to canonicalize workspace folder");

    return out_path;
}

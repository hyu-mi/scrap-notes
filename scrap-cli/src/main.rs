use clap::Parser;
use std::path::PathBuf;
use uuid::{Uuid, uuid};

mod app;
mod cli;

use app::App;
use cli::CliArgs;

fn main() {
    let args = CliArgs::parse();

    let workspace_dir = get_workspace_dir();
    let mut app = App::new(workspace_dir);
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

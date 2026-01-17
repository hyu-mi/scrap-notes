use clap::{Parser, Subcommand};
use scrap_core::{App, Command, CommandResponse};
use std::path::PathBuf;
use uuid::{Uuid, uuid};

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {
        title: String,
        #[arg(short, long, default_value = "plain-text")]
        file_type: String,
    },
    Folder {
        display_name: String,
        #[arg(short, long, default_value = "3e206920-6c75-7620-7520-6d722063656f")]
        parent_folder: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let workspace_dir = get_workspace_dir();
    let mut app = App::new(workspace_dir);

    let workspace_id = uuid!("3e206920-6c75-7620-7520-6d722063656f");

    let response = match cli.command {
        Commands::New { title, file_type } => app.handle_command(Command::CreateNote {
            parent: workspace_id,
            title,
            file_type,
        }),
        Commands::Folder {
            display_name,
            parent_folder,
        } => {
            let out_response = match Uuid::parse_str(&parent_folder) {
                Ok(id) => app.handle_command(Command::CreateFolder {
                    folder_id: id,
                    display_name,
                }),
                Err(_) => {
                    eprintln!("Error: '{}' is not a valid UUID.", parent_folder);
                    std::process::exit(1);
                }
            };
            out_response
        }
    };

    match response {
        CommandResponse::NoteCreated(id) => println!("[Scrap-core] Note created with ID: {}", id),
        CommandResponse::FolderCreated(id) => println!("[Scrap-core] Folder created with ID: {}", id),
        CommandResponse::InvalidCommand => eprintln!("[Scrap-core] invalid command!"),
        CommandResponse::Error(err) => eprintln!("[Scrap-core] Error: {:?}", err),
        _ => {}
    }
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

use std::fmt::Error;
use std::path::PathBuf;
use std::process::id;

use crate::app_command::AppCommand;
use crate::app_error::AppError;
use crate::app_event::AppEvent;
use crate::config::Config;
use crate::file_manager::FileManager;

pub struct App {
    config: Config,
    manager: FileManager,
}

impl App {
    pub fn new(config: Config) -> Self {
        let dir = config.workspace_dir.clone();
        App {
            config: config,
            manager: FileManager::new(dir),
        }
    }

    pub fn run(&mut self) {
        self.handle_command(AppCommand::CreateFile {
            name: String::from("Apple Pie"),
            file_type: String::from("Hearts2Hearts"),
        });
    }

    pub fn handle_command(self: &mut Self, command: AppCommand) -> Result<AppEvent, AppError> {
        match command {
            AppCommand::CreateFile { name, file_type } => {
                let result = self.manager.create_file(&name, &file_type);
                match result {
                    Ok(val) => return Ok(AppEvent::FileCreated { name: val }),
                    _ => return Err(AppError::IoError),
                }
            }
            AppCommand::OpenFile { name } => {
                return Ok(AppEvent::FileOpened {
                    content: String::from(""),
                });
            }
            AppCommand::SaveFile { name, content } => {
                return Ok(AppEvent::FileSaved);
            }
        }
    }
}

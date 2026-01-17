use crate::command::Command;
use crate::command_response::CommandResponse;
use crate::core::core::Core;
use crate::core::core_error::CoreError;
use crate::core::core_event::CoreEvent;
use crate::core::model::note;

use std::path::PathBuf;

pub struct App {
    core: Core,
}

impl App {
    pub fn new(workspace_dir: PathBuf) -> Self {
        return Self {
            core: Core::new(workspace_dir),
        };
    }

    pub fn run(self: &mut Self) {
        self.core.load_content();
    }

    pub fn handle_command(self: &mut Self, cmd: Command) -> CommandResponse {
        // Let's just say this was called seperatly...
        self.core.load_content();

        match cmd {
            Command::CreateNote {
                parent,
                title,
                file_type,
            } => match self.core.create_note(parent, &title, &file_type) {
                Ok(CoreEvent::NoteCreated(id)) => return CommandResponse::NoteCreated(id),
                Ok(_) => return CommandResponse::UnkownError("HOW???".to_string()),
                Err(err) => return CommandResponse::Error(err),
            },

            Command::CreateFolder {
                folder_id,
                display_name,
            } => match self.core.create_folder(folder_id, &display_name) {
                Ok(CoreEvent::FolderCreated(id)) => return CommandResponse::FolderCreated(id),
                Ok(_) => return CommandResponse::UnkownError("Ridiculous!".to_string()),
                Err(err) => return CommandResponse::Error(err),
            },
        }

        return CommandResponse::InvalidCommand;
    }
}

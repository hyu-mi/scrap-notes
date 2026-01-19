use crate::api::{ScrapCommand, ScrapError, ScrapEvent};
use crate::app::App;

use std::path::PathBuf;

pub struct Scrap {
    app: App,
}

impl Scrap {
    pub fn new(workspace_dir: PathBuf) -> Self {
        Self {
            app: App::new(workspace_dir),
        }
    }

    pub fn handle_command(self: &mut Self, cmd: ScrapCommand) -> Result<ScrapEvent, ScrapError> {
        match cmd {
            ScrapCommand::SyncWorkspace => {
                return self
                    .app
                    .load_workspace()
                    .map(ScrapEvent::from_app)
                    .map_err(ScrapError::from_app);
            }

            ScrapCommand::ListNotes => return Ok(ScrapEvent::NoteList(self.app.list_notes())),

            ScrapCommand::ListFolders => return Ok(ScrapEvent::FolderList(self.app.list_folders())),

            ScrapCommand::CreateNote {
                parent_id,
                title,
                file_type,
            } => {
                return self
                    .app
                    .create_note(parent_id, title, file_type)
                    .map(ScrapEvent::from_app)
                    .map_err(ScrapError::from_app);
            }

            ScrapCommand::CreateFolder {
                parent_id,
                display_name,
            } => {
                return self
                    .app
                    .create_folder(parent_id, display_name)
                    .map(ScrapEvent::from_app)
                    .map_err(ScrapError::from_app);
            }
        }

        return Err(ScrapError::NotImplemented(format!("Command is not implemented yet")));
    }
}

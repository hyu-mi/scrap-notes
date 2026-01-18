use crate::api::{FolderSummary, NoteSummary, ScrapCommand, ScrapResponse};
use crate::model::{Folder, FolderMetadata, Note};
use crate::workspace::{Workspace, WorkspaceError};
use crate::{ScrapError, ScrapEvent};

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use uuid::Uuid;
use uuid::uuid;

pub struct Scrap {
    workspace: Workspace,
    workspace_id: Uuid,
    notes: HashMap<Uuid, Note>,
    folders: HashMap<Uuid, Folder>,
}

impl Scrap {
    pub fn new(workspace_dir: PathBuf) -> Self {
        Self {
            workspace: Workspace::new(workspace_dir),
            workspace_id: uuid!("3e206920-6c75-7620-7520-6d722063656f"),
            notes: HashMap::new(),
            folders: HashMap::new(),
        }
    }

    pub fn handle_command(self: &mut Self, cmd: ScrapCommand) -> ScrapResponse {
        match cmd {
            ScrapCommand::SyncWorkspace => {
                //
                match self.load_content() {
                    Ok(_) => return ScrapResponse::WorkspaceSynced,
                    Err(err) => return ScrapResponse::Error(err),
                }
            }

            ScrapCommand::ListNotes => {
                return ScrapResponse::NoteList(self.list_notes());
            }

            ScrapCommand::ListFolders => {
                return ScrapResponse::FolderList(self.list_folders());
            }

            ScrapCommand::CreateNote {
                parent_id,
                title,
                file_type,
            } => {
                //
                match self.create_note(parent_id, &title, &file_type) {
                    Ok(ScrapEvent::NoteCreated(id)) => return ScrapResponse::NoteCreated(id),
                    Ok(_) => return ScrapResponse::UnkownError(format!("Invalid operation occured")),
                    Err(err) => return ScrapResponse::Error(err),
                }
            }

            ScrapCommand::CreateFolder {
                parent_id,
                display_name,
            } => {
                //
                match self.create_folder(parent_id, display_name) {
                    Ok(ScrapEvent::FolderCreated(id)) => return ScrapResponse::FolderCreated(id),
                    Ok(_) => return ScrapResponse::UnkownError("Ridiculous!".to_string()),
                    Err(err) => return ScrapResponse::Error(err),
                }
            }
        }
    }

    fn load_content(self: &mut Self) -> Result<ScrapEvent, ScrapError> {
        match self.workspace.scan_all(self.workspace_id.clone()) {
            Ok((mut loaded_notes, mut loaded_folders)) => {
                self.notes.extend(loaded_notes);
                self.folders.extend(loaded_folders);

                return Ok(ScrapEvent::ContentLoaded);
            }
            Err(err) => return Err(ScrapError::InvalidOperation("Could not load content :<")),
        }
    }

    fn print_content(self: &Self) {
        println!("");
        for (id, note) in &self.notes {
            note.print();
        }
        for (id, folder) in &self.folders {
            folder.print();
        }
    }

    fn list_notes(self: &Self) -> Vec<NoteSummary> {
        return self
            .notes
            .values()
            .map(|n| NoteSummary {
                id: n.get_id(),
                title: n.get_title().to_string(),
                file_type: n.get_file_type().to_string(),
            })
            .collect();
    }

    fn list_folders(self: &Self) -> Vec<FolderSummary> {
        return self
            .folders
            .values()
            .map(|f| FolderSummary {
                id: f.get_id(),
                display_name: f.get_display_name().to_string(),
            })
            .collect();
    }

    fn create_note(self: &mut Self, folder_id: Uuid, title: &str, file_type: &str) -> Result<ScrapEvent, ScrapError> {
        let parent_dir = self.get_directory(folder_id)?;

        match self.workspace.create_note(parent_dir, title, file_type) {
            Ok(note) => {
                let note_id = note.get_id();
                self.notes.insert(note_id.clone(), note);

                return Ok(ScrapEvent::NoteCreated(note_id));
            }
            Err(WorkspaceError::FileNameExhausted) => {
                return Err(ScrapError::NameCollision {
                    name: title.to_string(),
                    parent: Uuid::from(folder_id),
                });
            }
            Err(err) => {
                return Err(ScrapError::Unknown(format!("Workpace error: {:?}", err)));
            }
        }
    }

    fn get_note(self: &Self, id: Uuid) {}
    fn save_note(self: &Self, id: Uuid) {}
    fn delete_note(self: &Self, id: Uuid) {}

    fn create_folder(self: &mut Self, parent_id: Uuid, display_name: String) -> Result<ScrapEvent, ScrapError> {
        let parent_dir = self.get_directory(parent_id)?;

        match self.workspace.create_folder(parent_dir, &display_name, parent_id) {
            Ok(folder) => {
                let id = folder.get_id();
                self.folders.insert(id, folder);

                return Ok(ScrapEvent::FolderCreated(id));
            }
            Err(WorkspaceError::FolderNameExhausted) => {
                return Err(ScrapError::NameCollision {
                    name: display_name,
                    parent: Uuid::from(parent_id),
                });
            }
            Err(err) => {
                return Err(ScrapError::Unknown(format!("Workpace error: {:?}", err)));
            }
        }
    }

    fn get_directory(self: &Self, id: Uuid) -> Result<&Path, ScrapError> {
        if id == self.workspace_id {
            return Ok(Path::new(""));
        }

        return self
            .folders
            .get(&id)
            .map(|f| f.get_relative_path())
            .ok_or(ScrapError::FolderNotFound(id));
    }
}

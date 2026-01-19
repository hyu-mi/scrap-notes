use crate::api::{FolderSummary, NoteSummary};
use crate::app::{AppError, AppEvent};
use crate::model::{Folder, Note};
use crate::workspace::{Workspace, WorkspaceError};

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use uuid::uuid;

pub struct App {
    workspace: Workspace,
    workspace_id: Uuid,
    notes: HashMap<Uuid, Note>,
    folders: HashMap<Uuid, Folder>,
}

impl App {
    pub fn new(workspace_dir: PathBuf) -> Self {
        return Self {
            workspace: Workspace::new(workspace_dir),
            workspace_id: uuid!("3e206920-6c75-7620-7520-6d722063656f"),
            notes: HashMap::new(),
            folders: HashMap::new(),
        };
    }

    pub fn load_workspace(self: &mut Self) -> Result<AppEvent, AppError> {
        match self.workspace.scan_all(self.workspace_id.clone()) {
            Ok((mut loaded_notes, mut loaded_folders)) => {
                self.notes.extend(loaded_notes);
                self.folders.extend(loaded_folders);

                return Ok(AppEvent::WorkspaceLoaded);
            }
            Err(err) => {
                return Err(AppError::Unknown(format!(
                    "Failed to load workspace with error: {:?}",
                    err
                )));
            }
        }
    }

    pub fn list_notes(self: &Self) -> Vec<NoteSummary> {
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

    pub fn list_folders(self: &Self) -> Vec<FolderSummary> {
        return self
            .folders
            .values()
            .map(|f| FolderSummary {
                id: f.get_id(),
                display_name: f.get_display_name().to_string(),
            })
            .collect();
    }

    pub fn create_note(
        self: &mut Self,
        parent_id: Uuid,
        title: String,
        file_type: String,
    ) -> Result<AppEvent, AppError> {
        let parent_dir = self.get_directory(parent_id)?;

        match self.workspace.create_note(parent_dir, &title, &file_type) {
            Ok(note) => {
                let note_id = note.get_id();
                self.notes.insert(note_id.clone(), note);

                return Ok(AppEvent::NoteCreated(note_id));
            }

            Err(WorkspaceError::FileNameExhausted) => {
                return Err(AppError::NameCollision {
                    name: title,
                    parent: parent_id,
                });
            }

            Err(err) => {
                return Err(AppError::Unknown(format!("Workpace error: {:?}", err)));
            }
        }
    }

    // pub fn get_note(self: &Self, id: Uuid) {}
    // pub fn save_note(self: &Self, id: Uuid) {}
    // pub fn delete_note(self: &Self, id: Uuid) {}

    pub fn create_folder(self: &mut Self, parent_id: Uuid, display_name: String) -> Result<AppEvent, AppError> {
        let parent_dir = self.get_directory(parent_id)?;

        match self.workspace.create_folder(parent_dir, &display_name, parent_id) {
            Ok(folder) => {
                let id = folder.get_id();
                self.folders.insert(id, folder);

                return Ok(AppEvent::FolderCreated(id));
            }

            Err(WorkspaceError::FileNameExhausted) => {
                return Err(AppError::NameCollision {
                    name: display_name,
                    parent: parent_id,
                });
            }

            Err(err) => {
                return Err(AppError::Unknown(format!("Workpace error: {:?}", err)));
            }
        }
    }

    fn get_directory(self: &Self, id: Uuid) -> Result<&Path, AppError> {
        if id == self.workspace_id {
            return Ok(Path::new(""));
        }

        return self
            .folders
            .get(&id)
            .map(|f| f.get_relative_path())
            .ok_or(AppError::FolderNotFound(id));
    }
}

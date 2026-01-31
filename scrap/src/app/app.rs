use crate::api::{FolderSummary, NoteSummary};
use crate::app::{AppError, AppEvent};
use crate::index::{Index, IndexError};
use crate::model::{Folder, Note};
use crate::workspace::{Workspace, WorkspaceError};

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::result;
use uuid::Uuid;
use uuid::uuid;

pub struct App {
    workspace: Workspace,
    workspace_id: Uuid,
    index: Index,
}

impl App {
    pub fn new() -> Self {
        return Self {
            workspace: Workspace::new(),
            workspace_id: uuid!("3e206920-6c75-7620-7520-6d722063656f"),
            index: Index::new(),
        };
    }

    pub fn init(self: &mut Self, workspace_path: &Path) -> Result<(), AppError> {
        self.workspace
            .create_workspace(workspace_path)
            .map_err(|err| AppError::Unknown(format!("Workpace error: {:?}", err)))?;

        return Ok(());
    }

    pub fn load_workspace(self: &mut Self) -> Result<AppEvent, AppError> {
        let (loaded_notes, loaded_folders) = self
            .workspace
            .scan_workspace(self.workspace_id.clone())
            .map_err(|err| AppError::Unknown(format!("Failed to load workspace with error: {:?}", err)))?;

        // TODO: Map and handle index errors to app errors
        let notes_report = self.index.extend_notes(loaded_notes);
        let folders_report = self.index.extend_folders(loaded_folders);
        // TODO: Handle conflict reports

        return Ok(AppEvent::WorkspaceLoaded);
    }

    pub fn list_notes(self: &Self) -> Vec<NoteSummary> {
        return self.index.list_notes().unwrap_or_default();
    }

    pub fn list_folders(self: &Self) -> Vec<FolderSummary> {
        return self.index.list_folders().unwrap_or_default();
    }

    pub fn create_note(self: &mut Self, parent_id: Uuid, title: String, file_type: String) -> Result<Uuid, AppError> {
        let parent_dir = self.get_directory(parent_id)?;

        match self.workspace.create_note(parent_dir, &title, &file_type) {
            Ok(note) => {
                let note_id = note.get_id();

                self.index.insert_note(note);
                // TODO: Handle conflict reports
                // TODO: Map and handle index errors to app errors

                return Ok(note_id);
            }

            Err(err) => {
                return Err(AppError::Workspace(err));
            }
        }
    }

    pub fn remove_note(self: &mut Self, id: Uuid) -> Result<(), AppError> {
        // Update index
        let mut note_to_delete = self.index.remove_note(id).map_err(AppError::from_index)?;

        // Move note to trash
        self.workspace
            .move_note_to_trash(&mut note_to_delete)
            .map_err(|err| AppError::Unknown(format!("Workpace error: {:?}", err)))?;

        return Ok(());
    }

    pub fn get_note(self: &Self, id: Uuid) -> Result<String, AppError> {
        return self.index.get_note_body(id).map_err(AppError::from_index);
    }

    // pub fn save_note(self: &Self, id: Uuid) {}
    // pub fn delete_note(self: &Self, id: Uuid) {}

    pub fn create_folder(self: &mut Self, parent_id: Uuid, display_name: String) -> Result<Uuid, AppError> {
        let parent_dir = self.get_directory(parent_id)?;

        match self.workspace.create_folder(parent_dir, &display_name, parent_id) {
            Ok(folder) => {
                let folder_id = folder.get_id();

                self.index.insert_folder(folder);
                // TODO: Handle conflict reports
                // TODO: Map and handle index errors to app errors

                return Ok(folder_id);
            }

            Err(err) => {
                return Err(AppError::Unknown(format!("Workpace error: {:?}", err)));
            }
        }
    }

    //
    pub fn remove_folder(self: &mut Self, id: Uuid) -> Result<(), AppError> {
        // Update index
        let folder_to_delete = self.index.remove_folder(id).map_err(AppError::from_index)?;

        // Remove folder directory from workspace along with all the notes inside
        self.workspace
            .delete_folder(&folder_to_delete)
            .map_err(|err| AppError::Unknown(format!("Workpace error: {:?}", err)))?;

        return Ok(());
    }

    fn get_directory(self: &Self, id: Uuid) -> Result<&Path, AppError> {
        if id == self.workspace_id {
            return Ok(Path::new(""));
        }

        return self.index.get_folder_directory(id).map_err(AppError::from_index);
    }
}

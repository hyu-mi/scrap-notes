use crate::core::core_error::CoreError;
use crate::core::core_event::CoreEvent;
use crate::core::model::folder::Folder;
use crate::core::model::folder_metadata::FolderMetadata;
use crate::core::model::note::Note;
use crate::core::workspace::workspace::Workspace;
use crate::core::workspace::workspace_error::WorkspaceError;

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use uuid::Uuid;
use uuid::uuid;

pub struct Core {
    workspace: Workspace,
    workspace_id: Uuid,

    notes: HashMap<Uuid, Note>,
    folders: HashMap<Uuid, Folder>,
}

impl Core {
    pub fn new(workspace_dir: PathBuf) -> Self {
        Self {
            workspace: Workspace::new(workspace_dir),
            workspace_id: uuid!("3e206920-6c75-7620-7520-6d722063656f"),
            notes: HashMap::new(),
            folders: HashMap::new(),
        }
    }

    pub fn get_workspace_id(self: &Self) -> Uuid {
        return self.workspace_id.clone();
    }

    pub fn load_content(self: &mut Self) -> Result<CoreEvent, CoreError> {
        match self.workspace.scan_all(self.workspace_id.clone()) {
            Ok((mut loaded_notes, mut loaded_folders)) => {
                self.notes.extend(loaded_notes);
                self.folders.extend(loaded_folders);

                return Ok(CoreEvent::ContentLoaded);
            }
            Err(err) => return Err(CoreError::InvalidOperation("Could not load content :<")),
        }
    }

    pub fn create_note(self: &mut Self, folder_id: Uuid, title: &str, file_type: &str) -> Result<CoreEvent, CoreError> {
        let parent_dir = self.get_directory(folder_id)?;

        match self.workspace.create_note(parent_dir, title, file_type) {
            Ok(note) => {
                //
                let note_id = note.get_id();

                self.notes.insert(note_id.clone(), note);

                return Ok(CoreEvent::NoteCreated(note_id));
            }
            Err(WorkspaceError::NameExhausted) => {
                //
                return Err(CoreError::NameCollision {
                    name: title.to_string(),
                    parent: Uuid::from(folder_id),
                });
            }
            Err(err) => {
                //
                return Err(CoreError::Unknown(format!("Workpace error: {:?}", err)));
            }
        }
    }

    pub fn get_note(self: &Self, id: Uuid) {}

    pub fn save_note(self: &Self, id: Uuid) {}

    pub fn delete_note(self: &Self, id: Uuid) {}

    pub fn create_folder(self: &mut Self, folder_id: Uuid, display_name: &str) -> Result<CoreEvent, CoreError> {
        let parent_dir = self.get_directory(folder_id)?;

        match self.workspace.create_folder(parent_dir, display_name, folder_id) {
            Ok(folder) => {
                //
                let folder_id = folder.get_id();

                self.folders.insert(folder_id.clone(), folder);

                return Ok(CoreEvent::FolderCreated(folder_id));
            }
            Err(WorkspaceError::NameExhausted) => {
                //
                return Err(CoreError::NameCollision {
                    name: display_name.to_string(),
                    parent: Uuid::from(folder_id),
                });
            }
            Err(err) => {
                //
                return Err(CoreError::Unknown(format!("Workpace error: {:?}", err)));
            }
        }
    }

    fn get_directory(self: &Self, id: Uuid) -> Result<&Path, CoreError> {
        if id == self.workspace_id {
            return Ok(Path::new(""));
        }

        return self
            .folders
            .get(&id)
            .map(|f| f.get_relative_path())
            .ok_or(CoreError::FolderNotFound(id));
    }
}

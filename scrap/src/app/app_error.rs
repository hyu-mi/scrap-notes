use uuid::Uuid;

use crate::index::IndexError;
use crate::workspace::WorkspaceError;

#[derive(Debug)]
pub enum AppError {
    NameCollision { name: String, parent: Uuid },
    NoteNotFound(Uuid),
    FolderNotFound(Uuid),

    Workspace(WorkspaceError),
    Unknown(String),
}

impl AppError {
    pub fn from_index(err: IndexError) -> Self {
        match err {
            IndexError::NoteNotFound(id) => return Self::NoteNotFound(id),
            IndexError::FolderNotFound(id) => return Self::FolderNotFound(id),
            _ => return Self::Unknown(format!("SearchIndex error: {:?}", err)),
        }
    }
}

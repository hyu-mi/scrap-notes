use uuid::Uuid;

use crate::app::AppError;

#[derive(Debug)]
pub enum ScrapError {
    NoteNotFound(Uuid),
    FolderNotFound(Uuid),

    NotImplemented(String),
    Unknown(String),
}

impl ScrapError {
    pub fn from_app(err: AppError) -> Self {
        match err {
            AppError::NoteNotFound(id) => return Self::NoteNotFound(id),
            AppError::FolderNotFound(id) => return Self::FolderNotFound(id),

            AppError::Workspace(err) => return Self::Unknown(format!("Workspace Error: {:?}", err)),
            AppError::Unknown(msg) => return Self::Unknown(msg),
        }
    }
}

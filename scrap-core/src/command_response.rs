use uuid::Uuid;

use crate::core::core_error::CoreError;

pub enum CommandResponse {
    NoteCreated(Uuid),
    FolderCreated(Uuid),

    InvalidCommand,
    Error(CoreError),
    UnkownError(String),
}

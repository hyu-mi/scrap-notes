use crate::ScrapError;
use crate::api::{FolderSummary, NoteSummary};

use uuid::Uuid;

pub enum ScrapResponse {
    WorkspaceSynced,
    NoteList(Vec<NoteSummary>),
    FolderList(Vec<FolderSummary>),

    NoteCreated(Uuid),
    FolderCreated(Uuid),

    // Errors
    InvalidCommand,
    Error(ScrapError),
    UnkownError(String),
}

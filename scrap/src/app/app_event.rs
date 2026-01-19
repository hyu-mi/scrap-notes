use crate::api::{FolderSummary, NoteSummary};

use uuid::Uuid;

pub enum AppEvent {
    WorkspaceLoaded,
    NoteList(Vec<NoteSummary>),
    FolderList(Vec<FolderSummary>),

    NoteCreated(Uuid),
    FolderCreated(Uuid),
}

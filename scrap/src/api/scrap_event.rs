use crate::api::{FolderSummary, NoteSummary};
use crate::app::AppEvent;

use uuid::Uuid;

pub enum ScrapEvent {
    WorkspaceSynced,
    NoteList(Vec<NoteSummary>),
    FolderList(Vec<FolderSummary>),

    NoteCreated(Uuid),
    FolderCreated(Uuid),
}

impl ScrapEvent {
    pub fn from_app(event: AppEvent) -> Self {
        match event {
            AppEvent::WorkspaceLoaded => return Self::WorkspaceSynced,
            AppEvent::NoteList(out) => return Self::NoteList(out),
            AppEvent::FolderList(out) => return Self::FolderList(out),

            AppEvent::NoteCreated(id) => return Self::NoteCreated(id),
            AppEvent::FolderCreated(id) => return Self::FolderCreated(id),
        }
    }
}

use uuid::Uuid;

pub enum ScrapEvent {
    ContentLoaded,

    NoteCreated(Uuid),
    FolderCreated(Uuid),
}

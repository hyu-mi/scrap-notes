use uuid::Uuid;

pub enum CoreEvent {
    ContentLoaded,

    NoteCreated(Uuid),
    FolderCreated(Uuid),
}

impl CoreEvent {
    pub fn created_id(self: &Self) -> Option<Uuid> {
        match self {
            CoreEvent::NoteCreated(id) => Some(*id),
            CoreEvent::FolderCreated(id) => Some(*id),
            _ => None,
        }
    }
}

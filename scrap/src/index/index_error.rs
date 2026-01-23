use uuid::Uuid;

#[derive(Debug)]
pub enum IndexError {
    IdConflict(Uuid),

    NoteNotFound(Uuid),
    FolderNotFound(Uuid),

    NotFound,
}

use uuid::Uuid;

#[derive(Debug)]
pub enum ScrapError {
    NoteNotFound(Uuid),
    FolderNotFound(Uuid),

    NameCollision { name: String, parent: Uuid },

    InvalidOperation(&'static str),

    Unknown(String),
}

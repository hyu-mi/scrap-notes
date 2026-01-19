use uuid::Uuid;

#[derive(Debug)]
pub enum AppError {
    NameCollision { name: String, parent: Uuid },
    NoteNotFound(Uuid),
    FolderNotFound(Uuid),

    Unknown(String),
}

// impl AppError {
//     pub fn from_workspace(err: WorkspaceError) -> Self {
//         match err {
//             WorkspaceError::FolderNameExhausted => return Self::NameCollision { name: (), parent: () }
//         }
//     }
// }

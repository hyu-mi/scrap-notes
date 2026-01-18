use std::io;

#[derive(Debug)]
pub enum WorkspaceError {
    StorageFull,
    PermissionDenied,

    CorruptedFile,

    InvalidPath,
    NotFound,

    FolderMetadataAlreadyExists,

    Unknown(String),
    FileNameExhausted,
    FolderNameExhausted,
}

impl WorkspaceError {
    pub fn from_io(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::StorageFull => return Self::StorageFull,
            io::ErrorKind::PermissionDenied => return Self::PermissionDenied,
            io::ErrorKind::NotADirectory | io::ErrorKind::InvalidInput => return Self::InvalidPath,
            io::ErrorKind::NotFound => return Self::NotFound,
            io::ErrorKind::InvalidData => return Self::CorruptedFile,
            _ => return Self::Unknown(format!("{:?}", err)),
        }
    }
}

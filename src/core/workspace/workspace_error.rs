use std::io;

#[derive(Debug)]
pub enum WorkspaceError {
    StorageFull,
    PermissionDenied,
    InvalidPath,
    NotFound,
    NameExhausted,
    ReadError,
    Unknown(String),
}

impl WorkspaceError {
    pub fn from_io(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::StorageFull => return WorkspaceError::StorageFull,
            io::ErrorKind::PermissionDenied => return WorkspaceError::PermissionDenied,
            io::ErrorKind::NotADirectory | io::ErrorKind::InvalidInput => return WorkspaceError::InvalidPath,
            io::ErrorKind::NotFound => return WorkspaceError::NotFound,
            io::ErrorKind::AlreadyExists => return WorkspaceError::NameExhausted,
            _ => return WorkspaceError::Unknown(format!("{:?}", err)),
        }
    }
}

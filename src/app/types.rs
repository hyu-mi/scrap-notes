pub enum AppCommand {
    CreateFile { name: String, file_type: String },
    OpenFile { name: String },
    SaveFile { name: String, content: String },
}

pub enum AppEvent {
    FileCreated { name: String },
    FileOpened { content: String },
    FileSaved,
    FileLoaded,
}

#[derive(Debug)]
pub enum AppError {
    Unknown(String),
    InvalidCommand,

    // File Manager Errors
    NamingCollision,
    PermissionDenied,
}

impl AppError {
    pub fn from_io(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::PermissionDenied => return AppError::PermissionDenied,
            _ => AppError::Unknown(err.to_string()),
        }
    }
}

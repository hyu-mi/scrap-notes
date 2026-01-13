#[derive(Debug)]
pub enum FSError {
    NotADirectory(String),
    PermissionDenied(String),
    ParsingError(String),
    SecurityError(String),
    FileDoesNotExist,
    NameExhausted,
    Unknown(String),
}

impl FSError {
    pub fn from_io(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotADirectory => {
                return Self::NotADirectory("Parent Directly does not exists".to_string());
            }
            std::io::ErrorKind::PermissionDenied => {
                return Self::PermissionDenied("Could not get read write permission".to_string());
            }
            std::io::ErrorKind::InvalidFilename => {
                return Self::FileDoesNotExist;
            }
            _ => return Self::Unknown("".to_string()),
        }
    }
}

pub enum AppCommand {
    CreateFile { name: String, file_type: String },
    OpenFile { name: String },
    SaveFile { name: String, content: String },
}

pub enum AppEvent {
    FileCreated { name: String },
    FileOpened { content: String },
    FileSaved,
}

pub enum AppError {
    IoError,
    InvalidCommand,
}

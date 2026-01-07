pub enum AppEvent {
    FileCreated { name: String },
    FileOpened { content: String },
    FileSaved,
}

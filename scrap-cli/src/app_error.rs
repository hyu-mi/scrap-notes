#[derive(Debug)]
pub enum AppError {
    WorkspaceSyncFailed(String),
    ListNotesFailed(String),
    ListFoldersFailed(String),

    Invalid(String),
}

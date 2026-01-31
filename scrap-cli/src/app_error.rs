#[derive(Debug)]
pub enum AppError {
    WorkspaceInitializationFailed(String),
    WorkspaceSyncFailed(String),
    ListNotesFailed(String),
    ListFoldersFailed(String),

    Invalid(String),
}

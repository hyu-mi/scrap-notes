use crate::core::model::folder_metadata::FolderMetadata;
use std::path::PathBuf;
use uuid::Uuid;

pub struct Folder {
    metadata: FolderMetadata,
    path: PathBuf,
}

impl Folder {
    pub fn new(metadata: FolderMetadata, path: PathBuf) -> Self {
        return Self { metadata, path };
    }

    pub fn compose(self: &Self) -> String {
        let out = self.metadata.compose();
        return out;
    }
}

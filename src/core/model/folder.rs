use crate::core::{model::folder_metadata::FolderMetadata, parser::parse_folder::FolderData};
use std::path::PathBuf;
use uuid::Uuid;

pub struct Folder {
    metadata: FolderMetadata,
    relative_path: PathBuf,
    parent_id: Uuid,
}

impl Folder {
    pub fn new(relative_path: PathBuf, metadata: FolderMetadata, parent_id: Uuid) -> Self {
        return Self {
            metadata,
            relative_path,
            parent_id,
        };
    }

    pub fn from_data(relative_path: PathBuf, data: FolderData, parent_id: Uuid) -> Self {
        let id = data.id.unwrap_or_else(Uuid::new_v4);

        let display_name = data.display_name.unwrap_or_else(|| {
            relative_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string()
        });

        let metadata = FolderMetadata::new(id, display_name);

        return Self {
            metadata,
            relative_path,
            parent_id,
        };
    }

    pub fn get_id(self: &Self) -> Uuid {
        return self.metadata.get_id();
    }

    pub fn get_relative_path(self: &Self) -> PathBuf {
        return self.relative_path.clone();
    }

    pub fn compose(self: &Self) -> String {
        let out = self.metadata.compose();
        return out;
    }

    pub fn print(self: &Self) {
        let display_name = self.metadata.get_name();
        let id = self.metadata.get_id();

        println!(
            "Folder {}:\nid: {}\ndisplay-name: {}\n",
            &display_name,
            id.to_string(),
            &display_name
        );
    }
}

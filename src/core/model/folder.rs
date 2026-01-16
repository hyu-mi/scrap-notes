use crate::core::{model::folder_metadata::FolderMetadata, parser::parse_folder::FolderData};
use std::path::Path;
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

    pub fn get_relative_path(self: &Self) -> &Path {
        return &self.relative_path;
    }

    pub fn get_metadata_file_dir(self: &Self) -> PathBuf {
        return self.relative_path.join(".metadata.txt");
    }

    pub fn compose(self: &Self) -> String {
        return self.metadata.compose();
    }

    pub fn set_metadata(self: &mut Self, metadata: FolderMetadata) {
        self.metadata = metadata;
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

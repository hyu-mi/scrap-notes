use crate::model::FolderData;
use crate::model::FolderMetadata;

use std::path::Path;
use std::path::PathBuf;
use uuid::Uuid;

pub struct Folder {
    metadata: FolderMetadata,
    relative_path: PathBuf,
    parent_id: Uuid,
    child_notes: Vec<Uuid>,
    child_folders: Vec<Uuid>,
}

impl Folder {
    pub fn new(relative_path: PathBuf, metadata: FolderMetadata, parent_id: Uuid) -> Self {
        return Self {
            metadata,
            relative_path,
            parent_id,
            child_notes: Vec::new(),
            child_folders: Vec::new(),
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
            child_notes: Vec::new(),
            child_folders: Vec::new(),
        };
    }

    pub fn add_child_note(self: &mut Self, id: Uuid) {
        self.child_notes.push(id);
    }

    pub fn add_child_folder(self: &mut Self, id: Uuid) {
        self.child_folders.push(id);
    }

    pub fn get_child_notes(self: &Self) -> &Vec<Uuid> {
        return &self.child_notes;
    }

    pub fn get_child_folders(self: &Self) -> &Vec<Uuid> {
        return &self.child_folders;
    }

    pub fn get_id(self: &Self) -> Uuid {
        return self.metadata.get_id();
    }

    pub fn get_display_name(self: &Self) -> &str {
        return self.metadata.get_display_name();
    }

    pub fn get_relative_path(self: &Self) -> &Path {
        return &self.relative_path;
    }

    pub fn get_metadata_file_dir(self: &Self) -> PathBuf {
        return self.relative_path.join("_metadata.txt");
    }

    pub fn compose(self: &Self) -> String {
        return self.metadata.compose();
    }

    pub fn set_metadata(self: &mut Self, metadata: FolderMetadata) {
        self.metadata = metadata;
    }

    pub fn print(&self) {
        let display_name = self.metadata.get_display_name();
        let id = self.metadata.get_id().to_string();

        let yellow = "\x1b[33m";
        let gray = "\x1b[90m";
        let bold = "\x1b[1m";
        let reset = "\x1b[0m";

        let width = 60;
        let horiz = "─".repeat(width);

        println!("{gray}╭{}╮{reset}", horiz);

        let breadcrumb = format!("📂 /{} ({})", display_name, id);
        let padded_content = format!("{:<width$}", breadcrumb, width = width - 3);

        println!("{gray}│{reset} {yellow}{bold}{}{reset} {gray}│{reset}", padded_content);

        println!("{gray}╰{}╯{reset}", horiz);
    }
}

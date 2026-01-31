use crate::model::NoteData;
use crate::model::NoteMetadata;

use std::path::Path;
use std::path::PathBuf;
use uuid::Uuid;

pub struct Note {
    relative_path: PathBuf,
    metadata: NoteMetadata,
    body: String,
    is_dirty: bool,
    is_deleted: bool,
    // TODO: Need a last modified var so we can sync without opening the file...
}

impl Note {
    pub fn new(relative_path: PathBuf, metadata: NoteMetadata) -> Self {
        return Self {
            relative_path,
            metadata,
            body: String::new(),
            is_dirty: false,
            is_deleted: false,
        };
    }

    pub fn from_data(relative_path: PathBuf, data: NoteData) -> Self {
        let id = data.id.unwrap_or_else(Uuid::new_v4);

        let title = data.title.unwrap_or_else(|| {
            relative_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string()
        });

        // TODO: resolve file type by detecting it
        let file_type = data.file_type.unwrap_or("rich-text".to_string());

        let metadata = NoteMetadata::new(id, title, file_type);

        return Self {
            relative_path,
            metadata,
            body: data.body,
            is_dirty: false,
            is_deleted: false,
        };
    }

    pub fn get_relative_path(self: &Self) -> &Path {
        return &self.relative_path;
    }

    pub fn get_id(self: &Self) -> Uuid {
        return self.metadata.get_id();
    }

    pub fn get_title(self: &Self) -> &str {
        return self.metadata.get_title();
    }

    pub fn get_file_type(self: &Self) -> &str {
        return self.metadata.get_file_type();
    }

    pub fn get_body(self: &Self) -> &str {
        return &self.body;
    }

    pub fn mark_as_deleted(self: &mut Self) {
        self.is_deleted = true;
    }

    pub fn compose(self: &Self) -> String {
        let mut out = self.metadata.compose();
        out.push_str(&self.body);
        return out;
    }

    pub fn write_all(self: &mut Self, content: &str) {
        self.body = content.to_string();
    }
}

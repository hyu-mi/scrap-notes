use uuid::Uuid;

use crate::core::{model::note_metadata::NoteMetadata, parser::parse_note::NoteData};
use std::path::PathBuf;

pub struct Note {
    relative_path: PathBuf,
    metadata: NoteMetadata,
    body: String,
    is_dirty: bool,
}

impl Note {
    pub fn new(relative_path: PathBuf, metadata: NoteMetadata) -> Self {
        return Self {
            relative_path,
            metadata,
            body: String::new(),
            is_dirty: false,
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
        let file_type = data.file_type.unwrap_or_else(|| {
            relative_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("rich-text")
                .to_string()
        });

        let metadata = NoteMetadata::new(id, title, file_type);

        return Self {
            relative_path,
            metadata,
            body: data.body,
            is_dirty: false,
        };
    }

    pub fn get_relative_path(self: &Self) -> PathBuf {
        return self.relative_path.clone();
    }

    pub fn get_id(self: &Self) -> Uuid {
        return self.metadata.get_id();
    }

    pub fn compose(self: &Self) -> String {
        let mut out = self.metadata.compose();
        out.push_str(&self.body);
        return out;
    }

    pub fn write_all(self: &mut Self, content: &str) {
        self.body = content.to_string();
    }

    pub fn print(self: &Self) {
        let id = self.metadata.get_id();
        let title = self.metadata.get_title();
        let file_type = self.metadata.get_file_type();

        println!(
            "File {}:\nid: {}\ntitle: {}\nfile-type: {}\n",
            &title,
            id.to_string(),
            &title,
            &file_type
        );
    }
}

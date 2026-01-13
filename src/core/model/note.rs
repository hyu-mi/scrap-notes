use crate::core::model::note_metadata::NoteMetadata;
use std::path::PathBuf;

pub struct Note {
    metadata: NoteMetadata,
    path: PathBuf,
    content: String,
    is_dirty: bool,
}

impl Note {
    pub fn new(metadata: NoteMetadata, path: PathBuf) -> Self {
        Self {
            metadata,
            path,
            content: String::new(),
            is_dirty: false,
        }
    }

    pub fn get_path(self: &Self) -> PathBuf {
        return self.path.clone();
    }

    pub fn compose(self: &Self) -> String {
        let mut out = self.metadata.compose();
        out.push_str(&self.content);
        return out;
    }

    pub fn write_all(self: &mut Self, content: &str) {
        self.content = content.to_string();
    }
}

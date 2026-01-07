use crate::metadata::Metadata;

use std::path::PathBuf;

pub struct Note {
    pub metadata: Metadata,
    pub content: String,
    pub directory: PathBuf,
}

impl Note {
    pub fn new(metadata: Metadata, content: String, directory: PathBuf) -> Self {
        Self {
            metadata,
            content,
            directory,
        }
    }

    pub fn compose(self: &Self) -> String {
        String::from(format!(
            "#file-type: {}\n#created: {}\n#modified: {}\n---\n{}",
            self.metadata.file_type, self.metadata.created, self.metadata.modified, self.content
        ))
    }

    pub fn write_content(self: &mut Self, new_content: &str) {
        self.content.clear();
        self.content = new_content.to_string();
    }
}

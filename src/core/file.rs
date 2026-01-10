use std::path::PathBuf;

use uuid::Uuid;

pub struct File {
    pub id: Uuid,
    pub name: String,
    pub file_type: String,
    pub path: PathBuf,
    pub content: String,
    pub is_dirty: bool,
}

impl File {
    pub fn new(name: String, file_type: String, path: PathBuf, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            file_type,
            path,
            content,
            is_dirty: false,
        }
    }

    pub fn compose(self: &Self) -> Vec<u8> {
        let out = format!(
            "id: {}\ntype: {}\n\n-----------------------------------------\n\n{}",
            self.id, self.file_type, self.content
        );
        return out.into_bytes();
    }

    pub fn write_content(self: &mut Self, content: String) {
        self.content = content;
        self.is_dirty = true;
    }

    pub fn clear_dirty(self: &mut Self) {
        self.is_dirty = false;
    }
}

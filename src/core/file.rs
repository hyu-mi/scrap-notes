use std::path::PathBuf;

pub struct File {
    pub name: String,
    pub file_type: String,
    pub directory: PathBuf,
    pub content: String,
}

impl File {
    pub fn new(name: String, file_type: String, directory: PathBuf, content: String) -> Self {
        Self {
            name,
            file_type,
            directory,
            content,
        }
    }

    pub fn override_content(self: &mut Self, content: String) {
        self.content = content;
    }

    pub fn compose(self: &Self) -> Vec<u8> {
        let out = format!("#{}\n{}", self.file_type, self.content);
        return out.into_bytes();
    }
}

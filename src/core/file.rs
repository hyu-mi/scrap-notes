use std::path::{Path, PathBuf};
use std::{io::Read, str::FromStr};
use uuid::Uuid;

#[derive(Debug)]
pub struct File {
    pub id: Uuid,
    pub display_name: String,
    pub file_type: String,
    pub path: PathBuf,
    pub content: String,
    pub is_dirty: bool,
}

impl File {
    pub fn new(display_name: String, file_type: String, path: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4(),
            display_name,
            file_type,
            path,
            content: String::new(),
            is_dirty: false,
        }
    }

    pub fn compose(self: &Self) -> Vec<u8> {
        let out = format!(
            "id: {}\ndisplay-name: {}\ntype: {}\n---\n\n{}",
            &self.id, &self.display_name, &self.file_type, &self.content
        );
        return out.into_bytes();
    }

    pub fn parse(file: &mut std::fs::File, path: impl AsRef<Path>) -> Option<Self> {
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).ok()?;

        let (metadata, content) = file_content.split_once("---")?;

        let mut id: Option<Uuid> = None;
        let mut display_name: Option<&str> = None;
        let mut file_type: Option<&str> = None;

        for line in metadata.lines() {
            let trimmmed = line.trim();

            if let Some((token, value)) = trimmmed.split_once(':') {
                let clean_value = value.trim();

                // Empty values are invalid
                if clean_value.is_empty() {
                    continue;
                }

                match token.trim() {
                    "id" => id = Some(Uuid::from_str(clean_value).ok()?),
                    "display-name" => display_name = Some(clean_value),
                    "type" => file_type = Some(clean_value),
                    _ => {}
                }
            }
        }

        let (id, display_name, file_type) = (id?, display_name?, file_type?);
        return Some(Self {
            id,
            display_name: String::from(display_name),
            file_type: String::from(file_type),
            path: PathBuf::from(path.as_ref()),
            content: String::from(content),
            is_dirty: false,
        });
    }

    pub fn write_content(self: &mut Self, content: String) {
        self.content = content;
        self.is_dirty = true;
    }

    pub fn clear_dirty(self: &mut Self) {
        self.is_dirty = false;
    }
}

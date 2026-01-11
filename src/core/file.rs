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
            "id: \"{}\"\ndisplay-name: \"{}\"\ntype: \"{}\"\n---\n{}",
            &self.id, &self.display_name, &self.file_type, &self.content
        );
        return out.into_bytes();
    }

    fn extract_quoted(s: &str) -> Option<&str> {
        let start = s.find('"')?;
        let end = s.rfind('"')?;
        if start < end { Some(&s[start + 1..end]) } else { None }
    }

    pub fn parse(file: &mut std::fs::File, path: impl AsRef<Path>) -> Option<Self> {
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).ok()?;

        let mut id: Option<Uuid> = None;
        let mut display_name: Option<String> = None;
        let mut file_type: Option<String> = None;

        let mut offset = 0;
        let mut delimiter_found = false;
        for line in file_content.lines() {
            let line = line.trim();

            // Empty lines are ignored
            if line.is_empty() {
                continue;
            }

            offset += line.len() + 1;

            // Reached delimiter
            if line == "---" {
                delimiter_found = true;
                break;
            }

            // Maybe this is a metadata ?
            if let Some((key, value)) = line.split_once(':') {
                if let Some(extracted) = Self::extract_quoted(value) {
                    // Empty values are invalid
                    if extracted.is_empty() {
                        continue;
                    }

                    match key.trim() {
                        "id" => id = Some(Uuid::from_str(extracted).ok()?),
                        "display-name" => display_name = Some(extracted.to_string()),
                        "type" => file_type = Some(extracted.to_string()),
                        _ => {}
                    }
                }
            }
        }

        // TODO: Handle this
        if !delimiter_found {
            return None;
        }

        let (id, display_name, file_type) = (id?, display_name?, file_type?);

        let content = file_content[offset..].to_string();
        return Some(Self {
            id,
            display_name: display_name,
            file_type: file_type,
            path: PathBuf::from(path.as_ref()),
            content: content,
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

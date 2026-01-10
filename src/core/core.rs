use std::{collections::HashMap, path::PathBuf};
use uuid::Uuid;

use crate::core::{file::File, file_manager::FileManager};

pub struct Core {
    root: PathBuf,
    manager: FileManager,
    files: HashMap<Uuid, File>,
}

impl Core {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root: root,
            manager: FileManager::new(),
            files: HashMap::new(),
        }
    }

    fn save(file: &File) {
        FileManager::save_file(&file.path, &file.compose());
    }

    pub fn auto_save(self: &mut Self) {
        for file in self.files.values_mut().filter(|f| f.is_dirty) {
            Self::save(&file);
            file.clear_dirty();
        }
    }

    pub fn create_text(self: &mut Self, name: &str) -> std::io::Result<Uuid> {
        match FileManager::create_file(&self.root, name, "txt", String::from("rich-text")) {
            Ok(file) => {
                let id = file.id.clone();
                self.files.insert(id, file);
                return Ok(id);
            }
            Err(e) => return Err(e),
        }
    }

    pub fn save_file(self: &Self, id: &Uuid) -> std::io::Result<()> {
        match self.files.get(id) {
            Some(file) => {
                Self::save(file);
                return Ok(());
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No file found with the given id",
                ));
            }
        }
    }

    pub fn write_file_content(self: &mut Self, id: Uuid, content: String) {
        if let Some(file) = self.files.get_mut(&id) {
            file.write_content(content);
        }
    }
}

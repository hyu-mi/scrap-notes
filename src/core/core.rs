use uuid::Uuid;

use crate::core::{file::File, file_manager::FileManager};

use std::{collections::HashMap, path::PathBuf};

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

    pub fn create_text(self: &mut Self, name: &str) -> std::io::Result<Uuid> {
        match FileManager::create_file(&self.root, name, "txt") {
            Ok((directory, file)) => {
                let id = Uuid::new_v4();
                self.files.insert(
                    id,
                    File::new(
                        String::from(name),
                        String::from("plain-text"),
                        directory,
                        String::from("This was written by a code!"),
                    ),
                );
                return Ok(id);
            }
            Err(e) => return Err(e),
        }
    }

    pub fn save_file(self: &Self, id: &Uuid) -> std::io::Result<()> {
        match self.files.get(id) {
            Some(file) => {
                FileManager::save_file(&file.directory, &file.compose());
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
}

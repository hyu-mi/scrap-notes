use std::{collections::HashMap, path::PathBuf};
use uuid::Uuid;

use crate::{
    app::types::{AppError, AppEvent},
    core::{file::File, file_manager::FileManager, folder::Folder},
};

pub struct Core {
    manager: FileManager,
    files: HashMap<Uuid, File>,
    folders: HashMap<Uuid, Folder>,
}

impl Core {
    pub fn new(root: PathBuf) -> Self {
        Self {
            manager: FileManager::new(root),
            files: HashMap::new(),
            folders: HashMap::new(),
        }
    }

    fn save(file: &File) {
        FileManager::save_file(&file.path, &file.compose());
    }

    pub fn load_content(self: &mut Self) -> Result<AppEvent, AppError> {
        match self.manager.load_all() {
            Ok(files) => {
                for file in files {
                    self.files.insert(file.id.clone(), file);
                }
                return Ok(AppEvent::FileLoaded);
            }
            Err(e) => return Err(e),
        }
    }

    pub fn auto_save(self: &mut Self) {
        for file in self.files.values_mut().filter(|f| f.is_dirty) {
            Self::save(&file);
            file.clear_dirty();
        }
    }

    pub fn create_text(self: &mut Self, name: &str) -> Result<Uuid, AppError> {
        match self
            .manager
            .create_file(&PathBuf::new(), name, String::from("rich-text"))
        {
            Ok(file) => {
                // Also compose the newly created file
                Self::save(&file);

                let id: Uuid = file.id.clone();
                self.files.insert(id, file);

                return Ok(id);
            }
            Err(e) => return Err(e),
        }
    }

    // pub fn create_folder(self: &mut Self, name: &str) -> std::io::Result<Uuid> {
    //     match FileManager::create_folder(&self.root, name) {
    //         Ok(folder) => {
    //             let id = folder.id.clone();
    //             self.folders.insert(id, folder);
    //             return Ok(id);
    //         }
    //         Err(e) => return Err(e),
    //     }
    // }

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

    pub fn write_all(self: &mut Self, id: &Uuid, content: String) {
        if let Some(file) = self.files.get_mut(&id) {
            file.write_content(content);
        }
    }
}

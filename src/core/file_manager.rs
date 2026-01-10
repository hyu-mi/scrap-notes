use std::ffi::OsStr;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use crate::app::types::{AppError, AppEvent};
use crate::core::file::File;
use crate::core::folder::Folder;

pub struct FileManager {
    root: PathBuf,
}

impl FileManager {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn load_all(self: &Self) -> Result<Vec<File>, AppError> {
        let entries: fs::ReadDir = fs::read_dir(&self.root).expect("HOW???");

        let mut paths: Vec<PathBuf> = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                // Not a text file, yeet!
                if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("txt") {
                    continue;
                }

                paths.push(path);
            }
        }

        let mut files: Vec<File> = Vec::new();
        for path in paths {
            if let Ok(file) = Self::open_file(&path) {
                files.push(file);
            };
        }

        return Ok(files);
    }

    // pub fn create_folder(path: impl AsRef<Path>, name: &str) -> io::Result<Folder> {
    //     let base_path = path.as_ref();

    //     let mut folder_name: String = String::from(name);
    //     let mut folder_path: PathBuf = base_path.join(&folder_name);

    //     // Find a unique directory name
    //     let mut success: bool = false;

    //     if !folder_path.exists() {
    //         fs::create_dir(&folder_path)?;
    //         success = true;
    //     } else {
    //         for i in 1..256 {
    //             folder_name = format!("{name}_{i}");
    //             folder_path = base_path.join(&folder_name);

    //             if !folder_path.exists() {
    //                 fs::create_dir(&folder_path)?;
    //                 success = true;
    //                 break;
    //             }
    //         }
    //     }

    //     if !success {
    //         return Err(io::Error::new(
    //             io::ErrorKind::AlreadyExists,
    //             "Directory name exhausted.",
    //         ));
    //     }

    //     // Create folder metadata file
    //     let metadata_path = folder_path.join("folder_metadata.txt");
    //     fs::File::create(&metadata_path)?;

    //     return Ok(Folder::new(folder_name, String::new(), String::new(), folder_path));
    // }

    pub fn open_file(path: impl AsRef<Path>) -> Result<File, AppError> {
        match fs::OpenOptions::new().read(true).open(&path) {
            Ok(mut file) => match File::parse(&mut file, &path) {
                Some(file) => return Ok(file),
                _ => return Err(AppError::Unknown(String::from("Could not parse file content!"))),
            },
            Err(e) => return Err(AppError::from_io(e)),
        }
    }

    // ~
    pub fn create_file(
        self: &Self,
        sub_path: impl AsRef<Path>,
        name: &str,
        file_type: String,
    ) -> Result<File, AppError> {
        let mut path: PathBuf = self.root.join(&sub_path);

        match Self::get_valid_file_name(&path, name) {
            Ok(file_name) => {
                path = path.join(&file_name);
                OpenOptions::new().write(true).create(true).open(&path);
                return Ok(File::new(String::from(name), file_type, path));
            }
            Err(e) => return Err(e),
        }
    }

    // ~
    pub fn save_file(path: impl AsRef<Path>, content: &[u8]) -> Result<AppEvent, AppError> {
        match OpenOptions::new().write(true).create(true).truncate(true).open(path) {
            Ok(mut file) => {
                file.write_all(content);
                return Ok(AppEvent::FileSaved);
            }
            Err(e) => return Err(AppError::from_io(e)),
        }
    }

    // ~
    fn get_valid_file_name(dir: impl AsRef<Path>, name: &str) -> Result<String, AppError> {
        let path = dir.as_ref();
        let name: String = Self::slugify(name);

        let default_name: String = format!("{}.txt", &name);
        if !path.join(&default_name).exists() {
            return Ok(String::from(default_name));
        }

        // If default name is already taken, try appending numbers
        for i in 1..256 {
            let new_name: String = format!("{}_{}.txt", &name, i);
            if !path.join(&new_name).exists() {
                return Ok(new_name);
            }
        }

        return Err(AppError::NamingCollision);
    }

    // ~
    fn slugify(name: &str) -> String {
        return name
            .chars()
            .filter_map(|c| match c {
                ' ' | '_' => Some('-'),
                '-' => Some(c),
                '0'..='1' => Some(c),
                'A'..='Z' => Some(c.to_ascii_lowercase()),
                'a'..='z' => Some(c),
                _ => None,
            })
            .collect();
    }
}

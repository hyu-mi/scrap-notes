use core::error;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, ErrorKind, Read, Write};
use std::path::Path;
use std::path::PathBuf;
use std::{default, fs};

use crate::core::file::File;

pub struct FileManager {}

impl FileManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_file(path: impl AsRef<Path>, name: &str, extension: &str, file_type: String) -> io::Result<(File)> {
        let mut file_name: String = format!("{name}.{extension}");
        let mut file_path: PathBuf = path.as_ref().join(&file_name);

        let mut open_options = OpenOptions::new();
        open_options.write(true).create_new(true);

        if let Ok(file) = open_options.open(&file_path) {
            return Ok(File::new(file_name, file_type, file_path, String::new()));
        }

        // If the file with name already exists, we try appending a number for 256 times
        for i in 1..256 {
            file_name = format!("{name}_{i}.{extension}");
            file_path.set_file_name(&file_name);

            match open_options.open(&file_path) {
                Ok(file) => return Ok(File::new(file_name, file_type, file_path, String::new())),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(e),
            }
        }

        return Err(io::Error::new(io::ErrorKind::TimedOut, "File name exhausted."));
    }

    pub fn save_file(path: impl AsRef<Path>, content: &[u8]) -> io::Result<()> {
        match OpenOptions::new().write(true).create(true).truncate(true).open(path) {
            Ok(mut file) => {
                file.write_all(content);
                return Ok(());
            }
            Err(e) => return Err(e),
        }
    }
}

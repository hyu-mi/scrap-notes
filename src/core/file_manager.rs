use core::error;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, ErrorKind, Read, Write};
use std::path::Path;
use std::path::PathBuf;
use std::{default, fs};

pub struct FileManager {}

impl FileManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_file(directory: impl AsRef<Path>, name: &str, extension: &str) -> io::Result<(PathBuf, File)> {
        let mut path = directory.as_ref().join(format!("{name}.{extension}"));
        let mut open_options = OpenOptions::new();
        open_options.write(true).create_new(true);

        if let Ok(file) = open_options.open(&path) {
            return Ok((path, file));
        }

        for i in 1..256 {
            path.set_file_name(format!("{name}_{i}.{extension}"));

            match open_options.open(&path) {
                Ok(file) => return Ok((path, file)),
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

    // pub fn save_file(self: &mut Self, file_id: &str, content: &str) -> io::Result<()> {
    //     if let Some(note) = self.notes.get_mut(file_id) {
    //         // Update the note in memory
    //         note.write_content(content);

    //         // Write the updated note to the disk
    //         let mut file = File::create(&note.directory)?;
    //         file.write_all(note.compose().as_bytes())?;

    //         Ok(())
    //     } else {
    //         Err(io::Error::new(
    //             io::ErrorKind::Other,
    //             format!("No file with id: {} found!", file_id),
    //         ))
    //     }
    // }
}

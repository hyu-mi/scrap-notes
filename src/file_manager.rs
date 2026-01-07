use crate::metadata::{self, Metadata};
use crate::note::Note;

use core::error;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, ErrorKind, Read, Write};
use std::path::PathBuf;
use std::{default, fs};

pub struct FileManager {
    root_dir: PathBuf,
    notes: HashMap<PathBuf, Note>,
}

impl FileManager {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root_dir: root,
            notes: HashMap::new(),
        }
    }

    pub fn load_content(self: &mut Self) {
        // Retrieve all text files in the root directory
        let entries = fs::read_dir(&self.root_dir).expect("Could not read root directory");
        let mut files: Vec<PathBuf> = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                // Not a text file
                if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("txt") {
                    continue;
                }

                files.push(path);
            }
        }

        // Parse and add to cached notes
        for file in files {
            if let Ok(content) = fs::read_to_string(&file) {
                if let Some((metadata, content)) = Self::parse_content(&content) {
                    let note = Note::new(metadata, content.to_string(), file.clone());

                    self.notes.insert(file.clone(), note);
                } else {
                    // Should check file type and dates fletched
                    let note = Note::new(
                        Metadata::from(
                            "#plain-text".to_string(),
                            "2026-01-06".to_string(),
                            "2026-01-06".to_string(),
                        ),
                        content,
                        file.clone(),
                    );

                    self.notes.insert(file.clone(), note);
                }
            }
        }
    }

    fn parse_content(file_content: &str) -> Option<(Metadata, &str)> {
        let (metadata, content) = file_content.split_once("---")?;

        let mut file_type: Option<&str> = None;
        let mut created: Option<&str> = None;
        let mut modified: Option<&str> = None;

        for line in metadata.lines() {
            let trimmed = line.trim();

            // None metadata lines are discarded
            if !trimmed.starts_with("#") {
                continue;
            }

            if let Some((token, value)) = trimmed.split_once(':') {
                let trimmed_value = value.trim();

                // Empty values are discarded
                if trimmed_value.is_empty() {
                    continue;
                }

                match token.trim() {
                    "#file-type" => file_type = Some(trimmed_value),
                    "#created" => created = Some(trimmed_value),
                    "#modified" => modified = Some(trimmed_value),
                    _ => {}
                }
            }
        }

        let (file_type, created, modified) = (file_type?, created?, modified?);
        return Some((
            Metadata::from(
                file_type.to_string(),
                created.to_string(),
                modified.to_string(),
            ),
            content,
        ));
    }

    // Helper to print all cached notes
    pub fn print_all_notes(self: &Self) {
        for (id, note) in &self.notes {
            println!("");
            println!("file-type:{}", note.metadata.file_type);
            println!("created:{}", note.metadata.created);
            println!("modified:{}", note.metadata.modified);
            println!("content:{}", note.content);
        }
    }

    pub fn create_file(&mut self, name: &str, file_type: &str) -> io::Result<String> {
        // Create a new file at root directory
        let path = Self::get_empty_filename(&self.root_dir, name);
        let mut file = File::create(&path)?;

        // Update cached notes
        let note = Note::new(Metadata::new(file_type), String::new(), path.clone());

        // Compose the newly created file
        file.write_all(note.compose().as_bytes())?;

        self.notes.insert(path.clone(), note);

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "Invalid file name"))?
            .to_string();
        Ok(file_name)
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

    fn get_empty_filename(directory: &PathBuf, name: &str) -> PathBuf {
        let mut count: i32 = 0;
        let mut file_name: String = format!("{}.txt", name);
        loop {
            let path: PathBuf = directory.join(file_name);
            if !path.exists() {
                return path;
            }

            count += 1;
            file_name = format!("{}_{}.txt", name, count);
        }
    }
}

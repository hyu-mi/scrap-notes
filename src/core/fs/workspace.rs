use crate::core::fs::{fs_error::FSError, fs_event::FSEvent, fs_ops};
use crate::core::model::note_file::NoteFile;
use crate::core::model::{
    folder::Folder, folder_metadata::FolderMetadata, note::Note, note_metadata::NoteMetadata,
};
use crate::core::parser::{parse_file_to_note::parse_file_to_note, slugify::slugify};

use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct Workspace {
    workspace_root: PathBuf,
}

impl Workspace {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self { workspace_root }
    }

    pub fn scan_all_notes(self: &Self) -> Result<HashMap<Uuid, Note>, FSError> {
        let mut loaded_notes = HashMap::new();

        let entries = fs::read_dir(&self.workspace_root).map_err(FSError::from_io)?;

        for entry in entries {
            let entry = entry.map_err(FSError::from_io)?;
            let path = entry.path();

            // Not a text file
            if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("txt") {
                continue;
            }

            let relative_path = path.strip_prefix(&self.workspace_root).map_err(|_| {
                FSError::SecurityError("File found outside workspace root".to_string())
            })?;

            match self.open_note(relative_path) {
                Ok(note) => {
                    loaded_notes.insert(note.get_id(), note);
                }
                Err(e) => {
                    println!(
                        "Warning: Failed to load note at {:?}: {:?}",
                        relative_path, e
                    );
                    continue;
                }
            }
        }

        return Ok(loaded_notes);
    }

    pub fn create_note(
        self: &Self,
        parent_dir: &Path,
        title: &str,
        file_type: &str,
    ) -> Result<Note, FSError> {
        // Create text file
        let file = Self::create_unique_note_file(&self.workspace_root, parent_dir, title)?;

        // Create metadata for the note
        let metadata = NoteMetadata::new(Uuid::new_v4(), title, file_type);

        // Compose metadata as front matter
        fs_ops::write_file(&file.absolute_path, &metadata.compose())
            .map_err(FSError::from_io)
            .expect("");

        // Return Note
        return Ok(Note::new(metadata, file.relative_path));
    }

    pub fn save_note(self: &Self, note: &Note) -> Result<FSEvent, FSError> {
        let file_absolute_path = self.workspace_root.join(note.get_relative_path());
        let content = note.compose();

        match fs_ops::write_file(&file_absolute_path, &content) {
            Ok(_) => return Ok(FSEvent::FileSaved),
            Err(e) => return Err(FSError::from_io(e)),
        }
    }

    pub fn open_note(self: &Self, parent_dir: &Path) -> Result<Note, FSError> {
        let file_path = self.workspace_root.join(&parent_dir);

        let mut file = fs_ops::open_file(&file_path).map_err(FSError::from_io)?;

        if let Some(note) = parse_file_to_note(&mut file, PathBuf::from(parent_dir)) {
            return Ok(note);
        } else {
            return Err(FSError::ParsingError("ummm...".to_string()));
        }
    }

    pub fn create_folder(
        self: &Self,
        parent_dir: &Path,
        display_name: &str,
    ) -> Result<Folder, FSError> {
        // Create folder
        let folder_path =
            Self::create_unique_folder_directory(&self.workspace_root, parent_dir, display_name)?;

        // Create metadata text file in the folder directory
        let metadata_file_path = folder_path.join(".metadata.txt");

        let metadata_file = fs_ops::create_file(&metadata_file_path).map_err(FSError::from_io)?;

        // Create metadata for the Folder
        let metadata = FolderMetadata::new(Uuid::new_v4(), display_name);

        // Compose metadata
        fs_ops::write_file(&metadata_file_path, &metadata.compose()).map_err(FSError::from_io);

        // Return Folder struct
        return Ok(Folder::new(metadata, folder_path));
    }

    fn create_unique_note_file(
        workspace_root: &Path,
        parent_dir: &Path,
        title_name: &str,
    ) -> Result<NoteFile, FSError> {
        const MAX_COLLISION_RETRIES: u32 = 256;
        const FILE_EXTENSION: &str = "txt";

        let target_directory = workspace_root.join(parent_dir);
        if !target_directory.is_dir() {
            return Err(FSError::NotADirectory(format!(
                "Target directory '{:?}' does not exist",
                parent_dir
            )));
        }

        // Sanitize the input name to ensure valid file name
        let base_slug = slugify(title_name);
        let safe_slug = if base_slug.is_empty() {
            String::from("untitled-note")
        } else {
            base_slug
        };

        for i in 0..MAX_COLLISION_RETRIES {
            let file_name = if i == 0 {
                format!("{}.{}", safe_slug, FILE_EXTENSION)
            } else {
                format!("{}_{}.{}", safe_slug, i, FILE_EXTENSION)
            };

            let relative_file_path = parent_dir.join(&file_name);
            let absolute_file_path = workspace_root.join(&relative_file_path);

            match fs_ops::create_file(&absolute_file_path) {
                Ok(file) => {
                    // We return relative path
                    return Ok(NoteFile::new(
                        file_name,
                        relative_file_path,
                        absolute_file_path,
                    ));
                }
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    continue;
                }
                Err(e) => return Err(FSError::from_io(e)),
            }
        }

        return Err(FSError::NameExhausted);
    }

    fn create_unique_folder_directory(
        workspace_root: &Path,
        parent_dir: &Path,
        display_name: &str,
    ) -> Result<PathBuf, FSError> {
        const MAX_COLLISION_RETRIES: u32 = 256;

        let target_path = workspace_root.join(parent_dir);
        if !target_path.is_dir() {
            return Err(FSError::NotADirectory(format!(
                "Target directory '{:?}' does not exist",
                parent_dir
            )));
        }

        let base_slug = slugify(display_name);
        let safe_slug = if base_slug.is_empty() {
            String::from("untitled-folder")
        } else {
            base_slug
        };

        for i in 0..MAX_COLLISION_RETRIES {
            let folder_name = if i == 0 {
                safe_slug.clone()
            } else {
                format!("{}_{}", &safe_slug, i)
            };

            let absolute_folder_path = target_path.join(&folder_name);
            match fs::create_dir(&absolute_folder_path) {
                Ok(_) => return Ok(absolute_folder_path),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(FSError::from_io(e)),
            }
        }

        return Err(FSError::NameExhausted);
    }
}

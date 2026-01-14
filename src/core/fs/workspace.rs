use crate::core::fs::{fs_error::FSError, fs_event::FSEvent, fs_ops};

use crate::core::model::{
    folder::Folder, folder_metadata::FolderMetadata, note::Note, note_metadata::NoteMetadata,
};
use crate::core::parser::parse_folder::parse_folder;
use crate::core::parser::parse_note::parse_note;
use crate::core::text::slugify::slugify;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct Workspace {
    workspace_dir: PathBuf,
    id: Uuid,
}

impl Workspace {
    pub fn new(workspace_dir: PathBuf) -> Self {
        return Self {
            workspace_dir,
            id: Uuid::new_v4(),
        };
    }

    pub fn scan_all_notes(
        self: &Self,
    ) -> Result<(HashMap<Uuid, Note>, HashMap<Uuid, Folder>), FSError> {
        let mut notes = HashMap::new();
        let mut folders = HashMap::new();

        for entry in fs::read_dir(&self.workspace_dir).map_err(FSError::from_io)? {
            let entry = entry.map_err(FSError::from_io)?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            // Skip the metadata file itself
            if file_name_str == ".metadata.txt" {
                continue;
            }

            let entry_path = entry.path();
            let entry_relative_path = PathBuf::from(&file_name);

            if entry_path.is_file() {
                // Only process .txt files
                if entry_path.extension().and_then(|e| e.to_str()) == Some("txt") {
                    let note = Self::load_note_from(&self.workspace_dir, &entry_relative_path)?;
                    notes.insert(note.get_id(), note);
                }
            } else if entry_path.is_dir() {
                // Recurse into subfolder
                let (in_notes, in_folders) = Self::scan_folder_recursive(
                    &self.workspace_dir,
                    &entry_relative_path,
                    self.id.clone(),
                )?;
                notes.extend(in_notes);
                folders.extend(in_folders);
            }
        }

        return Ok((notes, folders));
    }

    pub fn create_note(
        self: &Self,
        parent_dir: &Path,
        title: &str,
        file_type: &str,
    ) -> Result<Note, FSError> {
        // Create text file for this note
        let file_path = Self::create_unique_note_file(&self.workspace_dir, parent_dir, title)?;

        // Create metadata for the note
        let metadata = NoteMetadata::new(Uuid::new_v4(), title, file_type);

        // Compose metadata as front matter
        fs_ops::write_file(&self.workspace_dir, &file_path, &metadata.compose())?;

        // Return Note
        return Ok(Note::new(file_path, metadata));
    }

    pub fn load_note(self: &Self, relative_path: &Path) -> Result<Note, FSError> {
        return Self::load_note_from(&self.workspace_dir, relative_path);
    }

    pub fn save_note(self: &Self, note: &Note) -> Result<FSEvent, FSError> {
        let note_path = note.get_relative_path();
        let content_to_save = note.compose();

        return fs_ops::write_file(&self.workspace_dir, &note_path, &content_to_save);
    }

    fn load_note_from(workspace_dir: &Path, relative_path: &Path) -> Result<Note, FSError> {
        let mut file = fs_ops::open_file(workspace_dir, relative_path)?;

        let mut raw_content = String::new();
        file.read_to_string(&mut raw_content)
            .ok()
            .ok_or(FSError::Unknown("Failed to read file content".to_string()))?;

        // Parse the note's file content to extract data
        let note_data = parse_note(raw_content);

        let note = Note::from_data(relative_path.to_path_buf(), note_data);
        return Ok(note);
    }

    fn create_unique_note_file(
        workspace_dir: &Path,
        parent_dir: &Path,
        title_name: &str,
    ) -> Result<PathBuf, FSError> {
        const MAX_COLLISION_RETRIES: u32 = 256;
        const FILE_EXTENSION: &str = "txt";

        // Target directory is not a directory
        let target_directory = workspace_dir.join(parent_dir);
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
            println!("{}", relative_file_path.display());

            match fs_ops::create_file(workspace_dir, &relative_file_path) {
                Ok(file) => {
                    // We return relative path
                    return Ok(relative_file_path);
                }
                Err(e) if e == FSError::AlreadyExist => {
                    continue;
                }
                Err(e) => return Err(e),
            }
        }

        return Err(FSError::NameExhausted);
    }

    fn scan_folder_recursive(
        workspace_dir: &Path,
        relative_path: &Path,
        parent_id: Uuid,
    ) -> Result<(HashMap<Uuid, Note>, HashMap<Uuid, Folder>), FSError> {
        let mut notes = HashMap::new();
        let mut folders = HashMap::new();

        let folder = Self::load_foler(workspace_dir, relative_path, parent_id);
        let folder_id = folder.get_id();
        folders.insert(folder.get_id(), folder);

        let full_dir_path = workspace_dir.join(relative_path);
        for entry in fs::read_dir(full_dir_path).map_err(FSError::from_io)? {
            let entry = entry.map_err(FSError::from_io)?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            // Skip the metadata file itself
            if file_name_str == ".metadata.txt" {
                continue;
            }

            let entry_path = entry.path();
            let entry_relative_path = relative_path.join(&file_name);

            if entry_path.is_file() {
                // Only process .txt files
                if entry_path.extension().and_then(|e| e.to_str()) == Some("txt") {
                    let note = Self::load_note_from(workspace_dir, &entry_relative_path)?;
                    notes.insert(note.get_id(), note);
                }
            } else if entry_path.is_dir() {
                // Recurse into subfolder
                let (in_notes, in_folders) =
                    Self::scan_folder_recursive(workspace_dir, &entry_relative_path, folder_id)?;
                notes.extend(in_notes);
                folders.extend(in_folders);
            }
        }

        return Ok((notes, folders));
    }

    fn load_foler(workspace_dir: &Path, relative_path: &Path, parent_id: Uuid) -> Folder {
        let metadata_path = relative_path.join(".metadata.txt");

        if let Ok(mut metadata_file) = fs_ops::open_file(workspace_dir, &metadata_path) {
            let mut raw_content = String::new();
            metadata_file.read_to_string(&mut raw_content);

            let data = parse_folder(raw_content);

            return Folder::from_data(relative_path.to_path_buf(), data, parent_id);
        }
        // If folder's metadata is missing, generate a new one
        // TODO: Handle warning to user later
        else {
            let display_name = relative_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string();
            let metadata = FolderMetadata::new(Uuid::new_v4(), display_name);

            // Also create the file in the directory
            // TODO: Handle errors later
            fs_ops::create_file(workspace_dir, &metadata_path);

            return Folder::new(relative_path.to_path_buf(), metadata, parent_id);
        }
    }
}

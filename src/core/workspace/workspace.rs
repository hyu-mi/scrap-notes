use crate::core::fs::fs_ops;
use crate::core::model::folder;
use crate::core::model::folder::Folder;
use crate::core::model::folder_metadata::FolderMetadata;
use crate::core::model::note::Note;
use crate::core::model::note_metadata::NoteMetadata;
use crate::core::parser::parse_folder::parse_folder;
use crate::core::parser::parse_note::parse_note;
use crate::core::text::slugify::slugify;
use crate::core::workspace::workspace_error::WorkspaceError;
use crate::core::workspace::workspace_event::WorkspaceEvent;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use uuid::Uuid;
use uuid::uuid;

pub struct Workspace {
    workspace_dir: PathBuf,
}

impl Workspace {
    pub fn new(workspace_dir: PathBuf) -> Self {
        return Self { workspace_dir };
    }

    pub fn scan_all(
        self: &Self,
        workspace_id: Uuid,
    ) -> Result<(HashMap<Uuid, Note>, HashMap<Uuid, Folder>), WorkspaceError> {
        return Self::load_directory(&self.workspace_dir, &PathBuf::new(), workspace_id)
            .map_err(WorkspaceError::from_io);
    }

    pub fn create_note(self: &Self, parent_dir: &Path, title: &str, file_type: &str) -> Result<Note, WorkspaceError> {
        return Self::create_note_at(&self.workspace_dir, Some(parent_dir), title, file_type);
    }

    pub fn save_note(self: &Self, note: &Note) -> Result<WorkspaceEvent, WorkspaceError> {
        let target_dir = note.get_relative_path();
        let content_to_save = note.compose();

        match fs_ops::write_file(&self.workspace_dir, &target_dir, &content_to_save) {
            Ok(_) => return Ok(WorkspaceEvent::NoteContentSaved),
            Err(err) => return Err(WorkspaceError::from_io(err)),
        }
    }

    pub fn load_note(self: &Self, target_dir: &Path) -> Result<Note, WorkspaceError> {
        return Self::load_note_at(&self.workspace_dir, target_dir).map_err(WorkspaceError::from_io);
    }

    pub fn create_folder(
        self: &Self,
        parent_dir: &Path,
        display_name: &str,
        parent_id: Uuid,
    ) -> Result<Folder, WorkspaceError> {
        return Self::create_folder_at(&self.workspace_dir, Some(parent_dir), display_name, parent_id);
    }

    pub fn save_folder(self: &Self, folder: &Folder) -> Result<WorkspaceEvent, WorkspaceError> {
        // Get the folder's metadata file
        let metadata_dir = folder.get_metadata_file_dir();
        let metadata_file = fs_ops::open_file(&self.workspace_dir, &metadata_dir).map_err(WorkspaceError::from_io)?;

        // Compose new data
        let content_to_save = folder.compose();
        fs_ops::write_file(&self.workspace_dir, &metadata_dir, &content_to_save).map_err(WorkspaceError::from_io)?;

        return Ok(WorkspaceEvent::FolderContentSaved);
    }

    pub fn load_folder(self: &Self, target_dir: &Path, parent_id: Uuid) -> Result<Folder, WorkspaceError> {
        match Self::load_folder_at(&self.workspace_dir, target_dir, parent_id) {
            Ok(folder) => return Ok(folder),
            Err(err) => return Err(WorkspaceError::from_io(err)),
        }
    }

    fn create_note_at(
        workspace_dir: &Path,
        parent_dir: Option<&Path>,
        title: &str,
        file_type: &str,
    ) -> Result<Note, WorkspaceError> {
        // Create text file for this note
        let file_path = Self::create_unique_note_file(workspace_dir, parent_dir, title)?;

        // Create metadata for the note
        let metadata = NoteMetadata::new(Uuid::new_v4(), title, file_type);

        // Compose metadata as front matter
        match fs_ops::write_file(workspace_dir, &file_path, &metadata.compose()) {
            Ok(_) => {}
            Err(err) => {
                // Try to remove the newly created text file
                let _ = fs_ops::delete_file(workspace_dir, &file_path);
                return Err(WorkspaceError::from_io(err));
            }
        }

        return Ok(Note::new(file_path, metadata));
    }

    fn load_note_at(workspace_dir: &Path, target_dir: &Path) -> std::io::Result<Note> {
        let mut file = fs_ops::open_file(workspace_dir, target_dir)?;

        let mut raw_content = String::new();
        file.read_to_string(&mut raw_content).ok().ok_or(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "Could not read file content",
        ))?;

        // Parse the note's file content to extract data
        let note_data = parse_note(raw_content);

        let note = Note::from_data(target_dir.to_path_buf(), note_data);
        return Ok(note);
    }

    fn create_folder_at(
        workspace_dir: &Path,
        parent_dir: Option<&Path>,
        display_name: &str,
        parent_id: Uuid,
    ) -> Result<Folder, WorkspaceError> {
        // Create folder directory
        let folder_dir = Self::create_unique_folder_dir(workspace_dir, parent_dir, display_name)?;

        // Create folder metadata file
        let metadata = match Self::create_folder_metadata(workspace_dir, &folder_dir, display_name) {
            Ok(metadata) => metadata,
            Err(err) => {
                // Try to remove the newly created folder directory
                fs_ops::delete_dir(workspace_dir, &folder_dir);

                return Err(WorkspaceError::from_io(err));
            }
        };

        return Ok(Folder::new(folder_dir.to_path_buf(), metadata, parent_id));
    }

    fn create_folder_metadata(
        workspace_dir: &Path,
        folder_dir: &Path,
        display_name: &str,
    ) -> std::io::Result<FolderMetadata> {
        let target_dir = folder_dir.join(".metadata.txt");

        // Create the folder's metadata text file
        fs_ops::create_file(workspace_dir, &target_dir)?;

        let metadata = FolderMetadata::new(Uuid::new_v4(), display_name);

        // Compose the metadata text file
        match fs_ops::write_file(workspace_dir, &target_dir, &metadata.compose()) {
            Ok(_) => return Ok(metadata),
            Err(err) => {
                // Try to remove the newly created metadata file
                let _ = fs_ops::delete_file(workspace_dir, &target_dir);
                return Err(err);
            }
        }
    }

    fn load_folder_at(workspace_dir: &Path, target_dir: &Path, parent_id: Uuid) -> std::io::Result<Folder> {
        // Get folder's metadata file
        let metadata_path = target_dir.join(".metadata.txt");
        let mut metadata_file = fs_ops::open_file(workspace_dir, &metadata_path)?;

        let mut file_content = String::new();

        metadata_file.read_to_string(&mut file_content)?;

        // Parse it
        let data = parse_folder(file_content);

        return Ok(Folder::from_data(target_dir.to_path_buf(), data, parent_id));
    }

    fn create_unique_note_file(
        workspace_dir: &Path,
        parent_dir: Option<&Path>,
        title_name: &str,
    ) -> Result<PathBuf, WorkspaceError> {
        const MAX_COLLISION_RETRIES: u32 = 256;
        const FILE_EXTENSION: &str = "txt";
        const MAX_FILENAME_LEN: usize = 64;

        // Sanitize the input name to ensure valid file name
        let base_slug = slugify(title_name);
        let safe_slug = if base_slug.is_empty() {
            String::from("untitled-note")
        } else {
            base_slug
        };
        let truncated_slug = if safe_slug.len() > MAX_FILENAME_LEN {
            safe_slug.chars().take(MAX_FILENAME_LEN).collect::<String>()
        } else {
            safe_slug
        };

        for i in 0..MAX_COLLISION_RETRIES {
            let file_name = if i == 0 {
                format!("{}.{}", truncated_slug, FILE_EXTENSION)
            } else {
                format!("{}_{}.{}", truncated_slug, i, FILE_EXTENSION)
            };

            let relative_file_path = if let Some(parent_dir) = parent_dir {
                parent_dir.join(&file_name)
            } else {
                PathBuf::from(&file_name)
            };

            match fs_ops::create_file(workspace_dir, &relative_file_path) {
                Ok(file) => {
                    // We return relative path
                    return Ok(relative_file_path);
                }
                Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                    continue;
                }
                Err(err) => return Err(WorkspaceError::from_io(err)),
            }
        }

        return Err(WorkspaceError::NameExhausted);
    }

    fn create_unique_folder_dir(
        workspace_dir: &Path,
        parent_dir: Option<&Path>,
        display_name: &str,
    ) -> Result<PathBuf, WorkspaceError> {
        const MAX_COLLISION_RETRIES: u32 = 256;
        const MAX_FOLDERNAME_LEN: usize = 32;

        // Sanitize the input name to ensure valid folder name
        let base_slug = slugify(display_name);
        let safe_slug = if base_slug.is_empty() {
            "untitled-folder".to_string()
        } else {
            base_slug
        };
        let truncated_slug = if safe_slug.len() > MAX_FOLDERNAME_LEN {
            safe_slug.chars().take(MAX_FOLDERNAME_LEN).collect::<String>()
        } else {
            safe_slug
        };

        for i in 0..MAX_COLLISION_RETRIES {
            let folder_name = if i == 0 {
                format!("{}", truncated_slug)
            } else {
                format!("{}_{}", truncated_slug, i)
            };

            let folder_relative_path = if let Some(parent_dir) = parent_dir {
                parent_dir.join(&folder_name)
            } else {
                PathBuf::from(&folder_name)
            };

            match fs_ops::create_dir(workspace_dir, &folder_relative_path) {
                Ok(_) => {
                    // We return relative path
                    return Ok(folder_relative_path);
                }
                Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                    continue;
                }
                Err(err) => return Err(WorkspaceError::from_io(err)),
            }
        }

        return Err(WorkspaceError::NameExhausted);
    }

    fn load_directory(
        workspace_dir: &Path,
        current_dir: &Path,
        parent_id: Uuid,
    ) -> std::io::Result<(HashMap<Uuid, Note>, HashMap<Uuid, Folder>)> {
        let mut notes = HashMap::new();
        let mut folders = HashMap::new();

        for entry in fs_ops::read_directory(workspace_dir, current_dir)? {
            let entry = entry?;
            let entry_name = entry.file_name();

            // Skip folder metadata files
            if entry_name.to_string_lossy() == ".metadata.txt" {
                continue;
            }

            let entry_path = entry.path();
            let relative_path = current_dir.join(&entry_name);

            // This is a file
            if entry_path.is_file() {
                // Note is just a text file
                if entry_path.extension().and_then(|e: &std::ffi::OsStr| e.to_str()) == Some("txt") {
                    let note = Self::load_note_at(workspace_dir, &relative_path)?;
                    notes.insert(note.get_id(), note);
                }
                continue;
            }

            // This is a folder
            if entry_path.is_dir() {
                let folder = Self::load_folder_or_create_metadata_and_load(workspace_dir, &relative_path, parent_id)?;

                let folder_id = folder.get_id();
                folders.insert(folder_id.clone(), folder);

                // Recurse into subfolder
                let (child_notes, child_folders) = Self::load_directory(workspace_dir, &relative_path, folder_id)?;
                notes.extend(child_notes);
                folders.extend(child_folders);
            }
        }

        return Ok((notes, folders));
    }

    fn load_folder_or_create_metadata_and_load(
        workspace_dir: &Path,
        folder_dir: &Path,
        parent_id: Uuid,
    ) -> std::io::Result<Folder> {
        match Self::load_folder_at(workspace_dir, folder_dir, parent_id) {
            Ok(folder) => return Ok(folder),

            // No metadata file found, create one for it
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                let display_name = folder_dir
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("untitled-folder");

                let metadata = Self::create_folder_metadata(workspace_dir, &folder_dir, display_name)?;

                return Ok(Folder::new(folder_dir.to_path_buf(), metadata, parent_id));
            }

            Err(err) => return Err(err),
        }
    }
}

use crate::fs::fs_ops;
use crate::model::{Folder, FolderData, FolderMetadata, Note, NoteData, NoteMetadata};
use crate::parser::{parse_folder::parse_folder, parse_note::parse_note};
use crate::text::{sanitize_name::sanitize_name, slugify::slugify};
use crate::workspace::{WorkspaceError, WorkspaceEvent};

use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use uuid::Uuid;
use uuid::uuid;

const MAX_COLLISION_RETRIES: u32 = 256;
const MAX_FILENAME_LEN: usize = 64;
const MAX_FOLDERNAME_LEN: usize = 32;
const METADATA_FILENAME: &str = "_metadata.txt";
const NOTE_FILE_EXTENSION: &str = "txt";

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
        return Self::load_directory(&self.workspace_dir, &PathBuf::new(), workspace_id);
    }

    /// Creates a new note with embedded metadata and saves it to the workspace.
    pub fn create_note(self: &Self, parent_dir: &Path, title: &str, file_type: &str) -> Result<Note, WorkspaceError> {
        let workspace_dir = &self.workspace_dir;

        // Reserve a unique path by creating an empty file
        let file_path = Self::create_unique_note_file(workspace_dir, parent_dir, title)?;

        // Prepare the metadata content
        let metadata = NoteMetadata::new(Uuid::new_v4(), title, file_type);
        let front_matter = metadata.compose();

        // Commit frontmatter to disk
        if let Err(err) = fs_ops::write_file(workspace_dir, &file_path, &front_matter) {
            // Try to delete the empty file
            let _ = fs_ops::delete_file(workspace_dir, &file_path);
            return Err(WorkspaceError::from_io(err));
        }

        return Ok(Note::new(file_path, metadata));
    }

    /// Saves note's content to the corresponding file in storage.
    /// TODO: changing note's title should trigger file rename to be consistent
    pub fn save_note(self: &Self, note: &Note) -> Result<WorkspaceEvent, WorkspaceError> {
        let target_dir = note.get_relative_path();
        let content_to_save = note.compose();

        if let Err(err) = fs_ops::write_file(&self.workspace_dir, &target_dir, &content_to_save) {
            return Err(WorkspaceError::from_io(err));
        }

        return Ok(WorkspaceEvent::NoteContentSaved);
    }

    /// Loads a note and it's metadata from the specified path.
    pub fn load_note(self: &Self, file_path: &Path) -> Result<Note, WorkspaceError> {
        let data = Self::load_note_data(&self.workspace_dir, file_path)?;

        return Ok(Note::from_data(file_path.to_path_buf(), data));
    }

    /// Creates a new folder with embedded metadata and saves it to the workspace.
    pub fn create_folder(
        self: &Self,
        parent_dir: &Path,
        display_name: &str,
        parent_id: Uuid,
    ) -> Result<Folder, WorkspaceError> {
        let workspace_dir = &self.workspace_dir;

        // reserve a unique folder directory
        let folder_dir = Self::create_unique_folder_dir(workspace_dir, parent_dir, display_name)?;

        // Create folder metadata file
        let metadata = match Self::create_folder_metadata(workspace_dir, &folder_dir, display_name) {
            Ok(metadata) => metadata,
            Err(err) => {
                // Try to remove the newly created folder directory
                fs_ops::delete_dir(workspace_dir, &folder_dir);

                return Err(err);
            }
        };

        return Ok(Folder::new(folder_dir.to_path_buf(), metadata, parent_id));
    }

    /// Saves folder's metadata content to the workspace.
    /// TODO: changing folder's display name should trigger folder rename
    pub fn save_folder(self: &Self, folder: &Folder) -> Result<WorkspaceEvent, WorkspaceError> {
        // Get the folder's metadata file
        let metadata_path = folder.get_metadata_file_dir();
        let metadata_file = fs_ops::open_file(&self.workspace_dir, &metadata_path).map_err(WorkspaceError::from_io)?;

        // Compose new data
        let content_to_save = folder.compose();
        fs_ops::write_file(&self.workspace_dir, &metadata_path, &content_to_save).map_err(WorkspaceError::from_io)?;

        return Ok(WorkspaceEvent::FolderContentSaved);
    }

    /// Loads a folder and it's metadata from the specified folder path.
    pub fn load_folder(self: &Self, folder_dir: &Path, parent_id: Uuid) -> Result<Folder, WorkspaceError> {
        let workspace_dir = &self.workspace_dir;

        let data = Self::load_folder_data(workspace_dir, folder_dir)?;

        return Ok(Folder::from_data(folder_dir.to_path_buf(), data, parent_id));
    }

    /// Reads and parses the raw disk content into a NoteData object.
    fn load_note_data(workspace_dir: &Path, file_path: &Path) -> Result<NoteData, WorkspaceError> {
        let mut file = fs_ops::open_file(workspace_dir, file_path).map_err(WorkspaceError::from_io)?;

        // Read file's content
        // TODO: Use ReadBuf!!!
        let mut raw_content = String::new();
        file.read_to_string(&mut raw_content).map_err(|err| match err.kind() {
            std::io::ErrorKind::PermissionDenied => WorkspaceError::PermissionDenied,
            std::io::ErrorKind::InvalidData => WorkspaceError::CorruptedFile,
            _ => WorkspaceError::from_io(err),
        })?;

        let data = parse_note(raw_content);

        return Ok(data);
    }

    /// Creates a new folder metadata file with embedded metadata and saves it to the workspace.
    /// Returns `FolderMetadataAlreadyExists` if the folder already has metadata.
    fn create_folder_metadata(
        workspace_dir: &Path,
        folder_dir: &Path,
        display_name: &str,
    ) -> Result<FolderMetadata, WorkspaceError> {
        let metadata_path = folder_dir.join(METADATA_FILENAME);

        // Create metadata file
        fs_ops::create_file(workspace_dir, &metadata_path).map_err(|err| match err.kind() {
            std::io::ErrorKind::AlreadyExists => WorkspaceError::FolderMetadataAlreadyExists,
            _ => WorkspaceError::from_io(err),
        })?;

        // Prepare the data
        let metadata = FolderMetadata::new(Uuid::new_v4(), display_name);
        let metadata_content = metadata.compose();

        // Commit metadata to disk
        if let Err(err) = fs_ops::write_file(workspace_dir, &metadata_path, &metadata_content) {
            // Try to remove the newly created metadata file if write fails
            let _ = fs_ops::delete_file(workspace_dir, &metadata_path);

            return Err(WorkspaceError::from_io(err));
        }

        return Ok(metadata);
    }

    /// Reads and parses the raw disk content into a FolderData object.
    fn load_folder_data(workspace_dir: &Path, folder_dir: &Path) -> Result<FolderData, WorkspaceError> {
        let metadata_path = folder_dir.join(METADATA_FILENAME);

        let mut metadata_file = fs_ops::open_file(workspace_dir, &metadata_path).map_err(WorkspaceError::from_io)?;

        // Read file's content
        let mut file_content = String::new();
        metadata_file
            .read_to_string(&mut file_content)
            .map_err(WorkspaceError::from_io)?;

        let data = parse_folder(file_content);

        return Ok(data);
    }

    /// Attempts to create a new note file
    /// until a unique path is found or the retry limit is reached.
    fn create_unique_note_file(
        workspace_dir: &Path,
        parent_dir: &Path,
        title_name: &str,
    ) -> Result<PathBuf, WorkspaceError> {
        // Sanitize the title name to ensure valid file name
        let base_name = sanitize_name(title_name, MAX_FILENAME_LEN);

        for i in 0..MAX_COLLISION_RETRIES {
            let filename = if i == 0 {
                format!("{}.{}", base_name, NOTE_FILE_EXTENSION)
            } else {
                format!("{}_{}.{}", base_name, i, NOTE_FILE_EXTENSION)
            };

            let relative_file_path = parent_dir.join(&filename);

            match fs_ops::create_file(workspace_dir, &relative_file_path) {
                Ok(file) => return Ok(relative_file_path),
                Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(err) => return Err(WorkspaceError::from_io(err)),
            }
        }

        return Err(WorkspaceError::FileNameExhausted);
    }

    /// Attempts to create a new folder directory
    /// until a unique path is found or the retry limit is reached.
    fn create_unique_folder_dir(
        workspace_dir: &Path,
        parent_dir: &Path,
        display_name: &str,
    ) -> Result<PathBuf, WorkspaceError> {
        // Sanitize the display name to ensure valid folder name
        let base_name = sanitize_name(display_name, MAX_FOLDERNAME_LEN);

        for i in 0..MAX_COLLISION_RETRIES {
            let foldername = if i == 0 {
                format!("{}", base_name)
            } else {
                format!("{}_{}", base_name, i)
            };

            let relative_folder_path = parent_dir.join(&foldername);

            match fs_ops::create_dir(workspace_dir, &relative_folder_path) {
                Ok(_) => return Ok(relative_folder_path),
                Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(err) => return Err(WorkspaceError::from_io(err)),
            }
        }

        return Err(WorkspaceError::FolderNameExhausted);
    }

    ///
    fn load_directory(
        workspace_dir: &Path,
        current_dir: &Path,
        parent_id: Uuid,
    ) -> Result<(HashMap<Uuid, Note>, HashMap<Uuid, Folder>), WorkspaceError> {
        let mut notes = HashMap::new();
        let mut folders = HashMap::new();

        for entry in fs_ops::read_directory(workspace_dir, current_dir).map_err(WorkspaceError::from_io)? {
            let entry = entry.map_err(WorkspaceError::from_io)?;
            let entry_name = entry.file_name();

            // Skip folder metadata files
            if entry_name.to_string_lossy() == METADATA_FILENAME {
                continue;
            }

            let entry_path = entry.path();
            let relative_path = current_dir.join(&entry_name);

            // This is a file
            if entry_path.is_file() {
                // Note is just a text file
                if entry_path.extension().and_then(|e: &std::ffi::OsStr| e.to_str()) == Some("txt") {
                    let note_data = Self::load_note_data(workspace_dir, &relative_path)?;

                    let note = Note::from_data(relative_path, note_data);
                    notes.insert(note.get_id(), note);
                }
                continue;
            }

            // This is a folder
            if entry_path.is_dir() {
                let folder_dir = relative_path;

                let folder_data = Self::load_folder_data(workspace_dir, &folder_dir)?;
                let folder = Folder::from_data(folder_dir.clone(), folder_data, parent_id);

                let folder_id = folder.get_id();
                folders.insert(folder_id.clone(), folder);

                // Recurse into subfolder
                let (child_notes, child_folders) = Self::load_directory(workspace_dir, &folder_dir, folder_id)?;
                notes.extend(child_notes);
                folders.extend(child_folders);
            }
        }

        return Ok((notes, folders));
    }
}

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

const FILENAME_LEN: usize = 64;
const FOLDERNAME_LEN: usize = 32;
const FILENAME_SEPARATOR_LEN: usize = 4; // The length of "____"
const UUID_LEN: usize = 36;

const MAX_FILENAME_LEN: usize = FILENAME_LEN + FILENAME_SEPARATOR_LEN + UUID_LEN;
const MAX_FOLDERNAME_LEN: usize = FOLDERNAME_LEN + FILENAME_SEPARATOR_LEN + UUID_LEN;

const METADATA_FILENAME: &str = "_metadata.txt";
const NOTE_FILE_EXTENSION: &str = "txt";

pub struct Workspace {
    workspace_dir: PathBuf,
}

impl Workspace {
    pub fn new(workspace_dir: PathBuf) -> Self {
        return Self { workspace_dir };
    }

    pub fn scan_workspace(self: &Self, workspace_id: Uuid) -> Result<(Vec<Note>, Vec<Folder>), WorkspaceError> {
        return Self::scan_directory(&self.workspace_dir, &PathBuf::new(), workspace_id);
    }

    /// Creates a new note with embedded metadata and saves it to the workspace.
    pub fn create_note(self: &Self, parent_dir: &Path, title: &str, file_type: &str) -> Result<Note, WorkspaceError> {
        let workspace_dir = &self.workspace_dir;

        let note_id = Uuid::new_v4();

        // Create unique note file using slug and ID
        let file_path = Self::create_note_file(workspace_dir, parent_dir, title, note_id)?;

        // Prepare the metadata content
        let metadata = NoteMetadata::new(note_id, title, file_type);
        let front_matter = metadata.compose();

        // Save frontmatter to disk
        if let Err(err) = fs_ops::write_file(workspace_dir, &file_path, &front_matter) {
            // Cleanup the empty note file if saving fails
            let _ = fs_ops::delete_file(workspace_dir, &file_path);

            return Err(WorkspaceError::from_io(err));
        }

        return Ok(Note::new(file_path, metadata));
    }

    /// Saves note's content to the corresponding file in storage.
    /// TODO: changing note's title should trigger file rename to be consistent
    pub fn save_note(self: &Self, note: &Note) -> Result<WorkspaceEvent, WorkspaceError> {
        let workspace_dir = &self.workspace_dir;

        let note_path = note.get_relative_path();
        let content_to_save = note.compose();

        fs_ops::write_file(workspace_dir, &note_path, &content_to_save).map_err(WorkspaceError::from_io)?;

        return Ok(WorkspaceEvent::NoteContentSaved);
    }

    /// Loads a note and it's metadata from the specified path.
    pub fn load_note(self: &Self, file_path: &Path) -> Result<Note, WorkspaceError> {
        let data = Self::load_note_data(&self.workspace_dir, file_path)?;

        return Ok(Note::from_data(file_path.to_path_buf(), data));
    }

    pub fn delete_note(self: &Self, note: &Note) -> Result<(), WorkspaceError> {
        let workspace_dir = &self.workspace_dir;

        let note_path = note.get_relative_path();

        fs_ops::delete_file(workspace_dir, note_path).map_err(WorkspaceError::from_io)?;

        return Ok(());
    }

    /// Creates a new folder with embedded metadata and saves it to the workspace.
    pub fn create_folder(
        self: &Self,
        parent_dir: &Path,
        display_name: &str,
        parent_id: Uuid,
    ) -> Result<Folder, WorkspaceError> {
        let workspace_dir = &self.workspace_dir;

        let folder_id = Uuid::new_v4();

        // Create unique folder directory using slug and ID
        let folder_dir = Self::create_folder_dir(workspace_dir, parent_dir, display_name, folder_id)?;
        let metadata_path = folder_dir.join(METADATA_FILENAME);

        // Create folder metadata file
        if let Err(err) = fs_ops::create_file(workspace_dir, &metadata_path) {
            // Cleanup folder if metadata creation fails
            let _ = fs_ops::delete_dir(workspace_dir, &folder_dir);

            return Err(WorkspaceError::from_io(err));
        }

        // Write metadata content on disk
        let metadata = FolderMetadata::new(folder_id, display_name);
        let metadata_content = metadata.compose();

        if let Err(err) = fs_ops::write_file(workspace_dir, &metadata_path, &metadata_content) {
            // Cleanup folder if metadata writing fails
            let _ = fs_ops::delete_dir(workspace_dir, &folder_dir);

            return Err(WorkspaceError::from_io(err));
        }

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

    pub fn delete_folder(self: &Self, folder: &Folder) -> Result<(), WorkspaceError> {
        let workspace_dir = &self.workspace_dir;

        let folder_dir = folder.get_relative_path();

        fs_ops::delete_dir(workspace_dir, folder_dir);

        return Ok(());
    }

    // TODO: folder를 로드할 때 하위에 포함된 모든 note도 함께 로드해야 하지 않나?
    // /// Loads a folder and it's metadata from the specified folder path.
    // pub fn load_folder(self: &Self, folder_dir: &Path, parent_id: Uuid) -> Result<Folder, WorkspaceError> {
    //     let workspace_dir = &self.workspace_dir;

    //     let data = Self::load_folder_data(workspace_dir, folder_dir)?;

    //     return Ok(Folder::from_data(folder_dir.to_path_buf(), data, parent_id));
    // }

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

    /// Creates a new note file with a name composed of
    /// the slugified title name and the unique note ID.
    fn create_note_file(
        workspace_dir: &Path,
        parent_dir: &Path,
        title_name: &str,
        note_id: Uuid,
    ) -> Result<PathBuf, WorkspaceError> {
        // Sanitize the title name to ensure valid file name
        let base_name = sanitize_name(title_name, MAX_FILENAME_LEN);

        let filename = format!("{}____{}.{}", base_name, note_id.to_string(), NOTE_FILE_EXTENSION);
        let relative_file_path = parent_dir.join(&filename);

        match fs_ops::create_dir(workspace_dir, &relative_file_path) {
            Ok(_) => return Ok(relative_file_path),
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => return Err(WorkspaceError::NameCollision),
            Err(err) => return Err(WorkspaceError::from_io(err)),
        }
    }

    /// Creates a new folder directory with a name composed of
    /// the slugified display name and the unique folder ID.
    fn create_folder_dir(
        workspace_dir: &Path,
        parent_dir: &Path,
        display_name: &str,
        folder_id: Uuid,
    ) -> Result<PathBuf, WorkspaceError> {
        // Sanitize the display name to ensure valid folder name
        let base_name = sanitize_name(display_name, MAX_FOLDERNAME_LEN);

        let folder_name = format!("{}____{}", base_name, folder_id.to_string());
        let relative_folder_dir = parent_dir.join(&folder_name);

        match fs_ops::create_dir(workspace_dir, &relative_folder_dir) {
            Ok(_) => return Ok(relative_folder_dir),
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => return Err(WorkspaceError::NameCollision),
            Err(err) => return Err(WorkspaceError::from_io(err)),
        }
    }

    ///
    fn scan_directory(
        workspace_dir: &Path,
        current_dir: &Path,
        parent_id: Uuid,
    ) -> Result<(Vec<Note>, Vec<Folder>), WorkspaceError> {
        let mut notes = Vec::new();
        let mut folders = Vec::new();

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
                    notes.push(note);
                }
                continue;
            }

            // This is a folder
            if entry_path.is_dir() {
                let folder_dir = relative_path;

                let folder_data = Self::load_folder_data(workspace_dir, &folder_dir)?;
                let mut folder = Folder::from_data(folder_dir.clone(), folder_data, parent_id);
                let folder_id = folder.get_id();

                // Recurse into subfolder
                let (child_notes, child_folders) = Self::scan_directory(workspace_dir, &folder_dir, folder_id)?;

                // Collect all notes whithin this folder
                for child_note in &child_notes {
                    folder.add_child_note(child_note.get_id());
                }

                // Collect all folders whithin this folder
                for child_folder in &child_folders {
                    folder.add_child_folder(child_folder.get_id());
                }

                folders.push(folder);

                notes.extend(child_notes);
                folders.extend(child_folders);
            }
        }

        return Ok((notes, folders));
    }
}

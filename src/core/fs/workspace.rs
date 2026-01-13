use crate::core::fs::{fs_error::FSError, fs_event::FSEvent, fs_ops};
use crate::core::model::note_file::NoteFile;
use crate::core::model::{
    folder::Folder, folder_metadata::FolderMetadata, note::Note, note_metadata::NoteMetadata,
};
use crate::core::parser::{parse_file_to_note::parse_file_to_note, slugify::slugify};

use std::fs::File;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn create_note(
        self: &Self,
        parent_dir: &Path,
        title: &str,
        file_type: &str,
    ) -> Result<Note, FSError> {
        // Create text file
        let file = Self::create_unique_note_file(&self.root, parent_dir, title)?;

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
        match fs_ops::write_file(&self.join(note.get_path()), &note.compose()) {
            Ok(_) => return Ok(FSEvent::FileSaved),
            Err(e) => return Err(FSError::from_io(e)),
        }
    }

    fn create_unique_note_file(
        workspace_root: &Path,
        parent_dir: &Path,
        title_name: &str,
    ) -> Result<NoteFile, FSError> {
        const MAX_COLLISION_RETRIES: u32 = 256;
        const FILE_EXTENSION: &str = "txt";

        // Sanitize the input name to ensure valid file name
        let base_slug = slugify(title_name);
        let safe_slug = if base_slug.is_empty() {
            "untitled".to_string()
        } else {
            base_slug
        };

        let target_directory = workspace_root.join(parent_dir);
        if !target_directory.is_dir() {
            return Err(FSError::NotADirectory(format!(
                "Target directory '{:?}' does not exist",
                parent_dir
            )));
        }

        for i in 0..MAX_COLLISION_RETRIES {
            let filename = if i == 0 {
                format!("{}.{}", safe_slug, FILE_EXTENSION)
            } else {
                format!("{}_{}.{}", safe_slug, i, FILE_EXTENSION)
            };

            let relative_file_path = parent_dir.join(&filename);
            let absolute_file_path = workspace_root.join(&relative_file_path);

            match fs_ops::create_file(&absolute_file_path) {
                Ok(file) => {
                    // We return relative path
                    return Ok(NoteFile::new(
                        filename,
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
}

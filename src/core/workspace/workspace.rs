use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use uuid::Uuid;
use uuid::uuid;

use crate::core::fs::fs_ops;
use crate::core::model::note::Note;
use crate::core::model::note_metadata::NoteMetadata;
use crate::core::parser::parse_note::parse_note;
use crate::core::text::slugify::slugify;
use crate::core::workspace::workspace_error::WorkspaceError;
use crate::core::workspace::workspace_event::WorkspaceEvent;

pub struct Workspace {
    workspace_dir: PathBuf,
    id: Uuid,
}

impl Workspace {
    pub fn new(workspace_dir: PathBuf) -> Self {
        return Self {
            workspace_dir,
            id: uuid!("3e206920-6c75-7620-7520-6d722063656f"),
        };
    }

    pub fn create_note(self: &Self, parent_dir: &Path, title: &str, file_type: &str) -> Result<Note, WorkspaceError> {
        // Create text file for this note
        let file_path = Self::create_unique_note_file(&self.workspace_dir, parent_dir, title)?;

        // Create metadata for the note
        let metadata = NoteMetadata::new(Uuid::new_v4(), title, file_type);

        // Compose metadata as front matter
        match fs_ops::write_file(&self.workspace_dir, &file_path, &metadata.compose()) {
            Ok(_) => {}
            Err(err) => {
                // Try to remove the newly created text file
                let _ = fs_ops::delete_file(&self.workspace_dir, &file_path);
                return Err(WorkspaceError::from_io(err));
            }
        }

        // Return Note
        return Ok(Note::new(file_path, metadata));
    }

    pub fn save_note(self: &Self, note: &Note) -> Result<WorkspaceEvent, WorkspaceError> {
        let target_dir = note.get_relative_path();
        let content_to_save = note.compose();

        match fs_ops::write_file(&self.workspace_dir, &target_dir, &content_to_save) {
            Ok(_) => return Ok(WorkspaceEvent::NoteSaved),
            Err(err) => return Err(WorkspaceError::from_io(err)),
        }
    }

    pub fn load_note(self: &Self, target_dir: &Path) -> Result<Note, WorkspaceError> {
        return Self::load_note_at(&self.workspace_dir, target_dir);
    }

    fn load_note_at(workspace_dir: &Path, target_dir: &Path) -> Result<Note, WorkspaceError> {
        let mut file = fs_ops::open_file(workspace_dir, target_dir).map_err(WorkspaceError::from_io)?;

        let mut raw_content = String::new();
        file.read_to_string(&mut raw_content)
            .ok()
            .ok_or(WorkspaceError::ReadError)?;

        // Parse the note's file content to extract data
        let note_data = parse_note(raw_content);

        let note = Note::from_data(target_dir.to_path_buf(), note_data);
        return Ok(note);
    }

    fn create_unique_note_file(
        workspace_dir: &Path,
        parent_dir: &Path,
        title_name: &str,
    ) -> Result<PathBuf, WorkspaceError> {
        const MAX_COLLISION_RETRIES: u32 = 256;
        const FILE_EXTENSION: &str = "txt";

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
}

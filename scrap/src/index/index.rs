use crate::api::{FolderSummary, NoteSummary};
use crate::index::{ExtendReport, IndexError, IndexEvent};
use crate::model::{Folder, Note};

use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

pub struct Index {
    notes: HashMap<Uuid, Note>,
    /// For searching notes by their title and type
    title_index: HashMap<String, Vec<Uuid>>,
    file_type_index: HashMap<String, Vec<Uuid>>,

    folders: HashMap<Uuid, Folder>,
    /// For searching for folders by their display name
    display_name_index: HashMap<String, Vec<Uuid>>,
}

impl Index {
    pub fn new() -> Self {
        return Self {
            notes: HashMap::new(),
            title_index: HashMap::new(),
            file_type_index: HashMap::new(),

            folders: HashMap::new(),
            display_name_index: HashMap::new(),
        };
    }

    /// Attempts to insert all provided notes into the index.
    ///
    /// ID conflicts are treated as non-fatal and are collected in the returned
    /// `ExtendNotesReport` and notes with conflicting IDs are skipped.
    ///
    /// Any other backend error (SQLite failures) causes the function to
    /// abort immediately and the error is returned.
    pub fn extend_notes(self: &mut Self, notes: Vec<Note>) -> Result<ExtendReport, IndexError> {
        let mut conflict_ids = Vec::new();
        let mut inserted_count: usize = 0;

        for note in notes {
            match self.insert_note(note) {
                Ok(_) => {
                    inserted_count += 1;
                }
                Err(IndexError::IdConflict(conflicted_id)) => {
                    conflict_ids.push(conflicted_id);
                }
                Err(err) => return Err(err),
            }
        }

        let report = ExtendReport::new(inserted_count, conflict_ids);
        return Ok(report);
    }

    /// Attempts to insert all provided folders into the index.
    ///
    /// ID conflicts are treated as non-fatal and are collected in the returned
    /// `ExtendNotesReport` and folders with conflicting IDs are skipped.
    ///
    /// Any other backend error (SQLite failures) causes the function to
    /// abort immediately and the error is returned.
    pub fn extend_folders(self: &mut Self, folders: Vec<Folder>) -> Result<ExtendReport, IndexError> {
        let mut conflict_ids = Vec::new();
        let mut inserted_count: usize = 0;

        for folder in folders {
            match self.insert_folder(folder) {
                Ok(_) => {
                    inserted_count += 1;
                }
                Err(IndexError::IdConflict(conflicted_id)) => {
                    conflict_ids.push(conflicted_id);
                }
                Err(err) => return Err(err),
            }
        }

        let report = ExtendReport::new(inserted_count, conflict_ids);
        return Ok(report);
    }

    pub fn insert_note(self: &mut Self, note: Note) -> Result<(), IndexError> {
        let id = note.get_id();

        if let Some(note) = self.notes.get(&id) {
            return Err(IndexError::IdConflict(id));
        }

        let title = note.get_title().to_string();
        let file_type = note.get_file_type().to_string();

        self.notes.insert(id, note);
        self.title_index.entry(title).or_default().push(id);
        self.file_type_index.entry(file_type).or_default().push(id);

        return Ok(());
    }

    pub fn insert_folder(self: &mut Self, folder: Folder) -> Result<(), IndexError> {
        let id = folder.get_id();

        if let Some(folder) = self.folders.get(&id) {
            return Err(IndexError::IdConflict(id));
        }

        let display_name = folder.get_display_name().to_string();

        self.folders.insert(id, folder);
        self.display_name_index.entry(display_name).or_default().push(id);

        return Ok(());
    }

    pub fn remove_note(self: &mut Self, id: Uuid) -> Result<Note, IndexError> {
        let note = self.notes.remove(&id).ok_or(IndexError::NoteNotFound(id))?;

        let title = note.get_title();
        if let Some(ids) = self.title_index.get_mut(title) {
            ids.retain(|v| *v != id);

            if ids.is_empty() {
                self.title_index.remove(title);
            }
        }

        let file_type = note.get_file_type();
        if let Some(ids) = self.file_type_index.get_mut(file_type) {
            ids.retain(|v| *v != id);

            if ids.is_empty() {
                self.file_type_index.remove(file_type);
            }
        }

        return Ok(note);
    }

    /// TODO: This function would remove the folder and relocate child notes to parent directory
    pub fn remove_folder(self: &mut Self, id: Uuid) -> Result<Folder, IndexError> {
        // Clear cached folder ids
        let folder = self.folders.remove(&id).ok_or(IndexError::FolderNotFound(id))?;

        let display_name = folder.get_display_name();
        if let Some(ids) = self.display_name_index.get_mut(display_name) {
            ids.retain(|v| *v != id);

            if ids.is_empty() {
                self.display_name_index.remove(display_name);
            }
        }

        // Remove all child notes
        for child_note in folder.get_child_notes() {
            self.remove_note(*child_note);
        }

        // recursive into child folders
        for child_folder in folder.get_child_folders() {
            self.remove_folder(*child_folder);
        }

        return Ok(folder);
    }

    pub fn list_notes(self: &Self) -> Result<Vec<NoteSummary>, IndexError> {
        return Ok(self
            .notes
            .values()
            .map(|n| NoteSummary::new(n.get_id().clone(), n.get_title(), n.get_file_type()))
            .collect());
    }

    pub fn list_folders(self: &Self) -> Result<Vec<FolderSummary>, IndexError> {
        return Ok(self
            .folders
            .values()
            .map(|f| FolderSummary::new(f.get_id().clone(), f.get_display_name()))
            .collect());
    }

    pub fn get_note(self: &Self, id: Uuid) -> Result<&Note, IndexError> {
        return self.notes.get(&id).ok_or(IndexError::NoteNotFound(id));
    }

    pub fn get_notes_by_title(self: &Self, title: &str) -> Result<Vec<&Note>, IndexError> {
        let ids = self.title_index.get(title).ok_or(IndexError::NotFound)?;

        let mut notes = Vec::new();

        for id in ids {
            if let Some(note) = self.notes.get(id) {
                notes.push(note);
            } else if cfg!(debug_assertions) {
                eprintln!("Database Error: found orphan id {} inside title_index!", id);
            }
        }

        return Ok(notes);
    }

    pub fn get_notes_by_type(self: &Self, file_type: &str) -> Result<Vec<&Note>, IndexError> {
        let ids = self.file_type_index.get(file_type).ok_or(IndexError::NotFound)?;

        let mut notes = Vec::new();

        for id in ids {
            if let Some(note) = self.notes.get(id) {
                notes.push(note);
            } else if cfg!(debug_assertions) {
                eprintln!("Database Error: found orphan id {} inside file_type_index!", id);
            }
        }

        return Ok(notes);
    }

    pub fn get_folder(self: &Self, id: Uuid) -> Result<&Folder, IndexError> {
        return self.folders.get(&id).ok_or(IndexError::FolderNotFound(id));
    }

    pub fn get_folders_by_display_name(self: &Self, display_name: &str) -> Result<Vec<&Folder>, IndexError> {
        let ids = self.display_name_index.get(display_name).ok_or(IndexError::NotFound)?;

        let mut folders = Vec::new();

        for id in ids {
            if let Some(folder) = self.folders.get(id) {
                folders.push(folder);
            } else if cfg!(debug_assertions) {
                eprintln!("Database Error: found orphan id {} inside display_name_index!", id);
            }
        }

        return Ok(folders);
    }

    pub fn get_note_body(self: &Self, id: Uuid) -> Result<String, IndexError> {
        let note = self.notes.get(&id).ok_or(IndexError::NoteNotFound(id))?;

        return Ok(note.get_body().to_string());
    }

    pub fn get_note_directory(self: &Self, id: Uuid) -> Result<&Path, IndexError> {
        return self
            .notes
            .get(&id)
            .map(|n| n.get_relative_path())
            .ok_or(IndexError::NoteNotFound(id));
    }

    pub fn get_folder_directory(self: &Self, id: Uuid) -> Result<&Path, IndexError> {
        return self
            .folders
            .get(&id)
            .map(|f| f.get_relative_path())
            .ok_or(IndexError::FolderNotFound(id));
    }
}

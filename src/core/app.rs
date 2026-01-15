use crate::core::model::note::Note;
use crate::core::workspace::workspace::Workspace;

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use uuid::Uuid;

pub struct App {
    workspace: Workspace,
}

impl App {
    pub fn new(root: PathBuf) -> Self {
        Self {
            workspace: Workspace::new(root),
        }
    }

    pub fn run(self: &Self) {
        // Create new note
        let mut new_note = self
            .workspace
            .create_note(&PathBuf::new(), "Loona", "rich-text")
            .expect("Failed to create new note");

        new_note.write_all("Boy I, boy I, boy I know");

        self.workspace.save_note(&new_note).expect("Failed to save new note");

        // Load an existing note
        let existing_note_path = PathBuf::from("loona.txt");
        let mut existing_note = self
            .workspace
            .load_note(&existing_note_path)
            .expect("Failed to load existing note");

        existing_note.write_all("Writing an existing note");

        self.workspace
            .save_note(&existing_note)
            .expect("Failed to save existing ntoe");
    }
}

use uuid::Uuid;

use crate::core::{fs::workspace::Workspace, model::note::Note};
use std::{collections::HashMap, path::PathBuf, str::FromStr};

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
        let mut notes = self
            .workspace
            .scan_all_notes()
            .expect("Failed to load content in the workspace directory");
        for (id, note) in &notes {
            println!("note id {} composed:\n{}", id, note.compose());
        }

        {
            // Create a new note
            let mut note = self
                .workspace
                .create_note(&PathBuf::new(), "상추", "rich-text")
                .expect("Failed to create note");

            note.write_all("I wrote this, I am danger");

            self.workspace
                .save_note(&note)
                .expect("Failed to save note");
            notes.insert(note.get_id(), note);
        }

        {
            // Access already existing note
            let id_string = "9ca43694-f314-46fe-b024-154030e41353";
            let existing_id = Uuid::from_str(&id_string)
                .expect(&format!("id '{}' is not a valid uuid id", id_string));
            let mut existing_note = notes
                .get_mut(&existing_id)
                .expect(&format!("No existing note found for id: {}", id_string));

            existing_note.write_all("Writing an already existing note!");

            self.workspace.save_note(&existing_note);
        }

        // Create a folder
        self.workspace.create_folder(&PathBuf::new(), "BABYMONSTER");
    }
}

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
        }

        let (mut notes, mut folders) = self
            .workspace
            .scan_all_notes()
            .expect("Failed to scan all notes");

        for (id, folder) in folders {
            folder.print();
        }

        for (id, note) in notes {
            note.print();
        }
    }
}

use crate::core::fs::workspace::Workspace;
use std::path::PathBuf;

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
        let mut note = self
            .workspace
            .create_note(&PathBuf::new(), "상추", "rich-text")
            .expect("Failed to create note");

        note.write_all("I wrote this, I am danger");

        self.workspace
            .save_note(&note)
            .expect("Failed to save note");
    }
}

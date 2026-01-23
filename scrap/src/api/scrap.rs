use crate::api::{FolderSummary, NoteSummary, ScrapError};
use crate::app::{App, AppEvent};

use std::path::PathBuf;
use uuid::Uuid;

pub struct Scrap {
    app: App,
}

impl Scrap {
    pub fn new(workspace_dir: PathBuf) -> Self {
        Self {
            app: App::new(workspace_dir),
        }
    }

    pub fn sync_workspace(self: &mut Self) -> Result<(), ScrapError> {
        return self.app.load_workspace().map(|_| ()).map_err(ScrapError::from_app);
    }

    pub fn list_notes(self: &mut Self) -> Result<Vec<NoteSummary>, ScrapError> {
        return Ok(self.app.list_notes());
    }

    pub fn list_folders(self: &mut Self) -> Result<Vec<FolderSummary>, ScrapError> {
        return Ok(self.app.list_folders());
    }

    pub fn create_note(self: &mut Self, parent_id: Uuid, title: String, file_type: String) -> Result<Uuid, ScrapError> {
        return self
            .app
            .create_note(parent_id, title, file_type)
            .map_err(ScrapError::from_app);
    }

    pub fn create_folder(self: &mut Self, parent_id: Uuid, display_name: String) -> Result<Uuid, ScrapError> {
        return self
            .app
            .create_folder(parent_id, display_name)
            .map_err(ScrapError::from_app);
    }

    // pub fn get_note(self: &mut Self) -> Result<(), ScrapError> {
    //     //
    // }
}

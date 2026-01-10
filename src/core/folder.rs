use std::path::PathBuf;
use uuid::Uuid;

pub struct Folder {
    pub id: Uuid,
    pub display_name: String,
    pub color: String,
    pub icon: String,
    pub path: PathBuf,
}

impl Folder {
    pub fn new(display_name: String, color: String, icon: String, path: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4(),
            display_name,
            color,
            icon,
            path,
        }
    }
}

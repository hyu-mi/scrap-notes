use uuid::Uuid;

pub struct FolderData {
    pub id: Option<Uuid>,
    pub display_name: Option<String>,
}

impl FolderData {
    pub fn new() -> Self {
        return Self {
            id: None,
            display_name: None,
        };
    }
}

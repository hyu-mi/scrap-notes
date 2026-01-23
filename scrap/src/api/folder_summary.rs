use uuid::Uuid;

pub struct FolderSummary {
    pub id: Uuid,
    pub display_name: String,
}

impl FolderSummary {
    pub fn new(id: Uuid, display_name: impl Into<String>) -> FolderSummary {
        return Self {
            id,
            display_name: display_name.into(),
        };
    }
}

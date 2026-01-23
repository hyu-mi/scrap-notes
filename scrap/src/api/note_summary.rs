use uuid::Uuid;

pub struct NoteSummary {
    pub id: Uuid,
    pub title: String,
    pub file_type: String,
}

impl NoteSummary {
    pub fn new(id: Uuid, title: impl Into<String>, file_type: impl Into<String>) -> Self {
        return Self {
            id,
            title: title.into(),
            file_type: file_type.into(),
        };
    }
}

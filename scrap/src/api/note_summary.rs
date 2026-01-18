use uuid::Uuid;

pub struct NoteSummary {
    pub id: Uuid,
    pub title: String,
    pub file_type: String,
}

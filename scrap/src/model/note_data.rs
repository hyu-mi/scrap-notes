use uuid::Uuid;

pub struct NoteData {
    pub id: Option<Uuid>,
    pub title: Option<String>,
    pub file_type: Option<String>,
    pub body: String,
}

impl NoteData {
    pub fn new() -> Self {
        return Self {
            id: None,
            title: None,
            file_type: None,
            body: String::new(),
        };
    }
}

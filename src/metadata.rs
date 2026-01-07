pub struct Metadata {
    pub file_type: String,
    pub created: String,
    pub modified: String,
}

impl Metadata {
    pub fn new(file_type: &str) -> Self {
        Self {
            file_type: String::from(file_type),
            created: String::from("2026-01-06"),
            modified: String::from("2026-01-06"),
        }
    }

    pub fn from(file_type: String, created: String, modified: String) -> Self {
        Self {
            file_type,
            created,
            modified,
        }
    }
}

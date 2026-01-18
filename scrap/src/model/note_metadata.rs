use uuid::Uuid;

pub struct NoteMetadata {
    id: Uuid,
    title: String,
    file_type: String,
}

impl NoteMetadata {
    pub fn new(id: Uuid, title: impl Into<String>, file_type: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            file_type: file_type.into().to_ascii_lowercase(),
        }
    }

    pub fn get_id(self: &Self) -> Uuid {
        return self.id.clone();
    }

    pub fn get_title(self: &Self) -> &str {
        return &self.title;
    }

    pub fn get_file_type(self: &Self) -> &str {
        return &self.file_type;
    }

    pub fn compose(self: &Self) -> String {
        return format!(
            "---\nid: \"{}\"\ntitle: \"{}\"\ntype: \"{}\"\n---\n",
            self.id.to_string(),
            self.title,
            self.file_type
        );
    }
}

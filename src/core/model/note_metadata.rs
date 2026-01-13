use uuid::Uuid;

pub struct NoteMetadata {
    id: Uuid,
    title: String,
    file_type: String,
}

impl NoteMetadata {
    pub fn new(id: Uuid, title: &str, file_type: &str) -> Self {
        Self {
            id,
            title: String::from(title),
            file_type: String::from(file_type),
        }
    }

    pub fn get_id(self: &Self) -> Uuid {
        return self.id.clone();
    }

    pub fn compose(self: &Self) -> String {
        return format!(
            "id: \"{}\"\ntitle: \"{}\"\ntype: \"{}\"\n---\n",
            self.id.to_string(),
            self.title,
            self.file_type
        );
    }
}

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

    pub fn compose(self: &Self) -> String {
        return format!(
            "id: \"{}\"\ntitle: \"{}\"\ntype: \"{}\"\n---\n",
            self.id, self.title, self.file_type
        );
    }
}

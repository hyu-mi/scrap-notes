use uuid::Uuid;

pub struct FolderMetadata {
    id: Uuid,
    display_name: String,
}

impl FolderMetadata {
    pub fn new(id: Uuid, display_name: &str) -> Self {
        return Self {
            id,
            display_name: display_name.to_string(),
        };
    }

    pub fn compose(self: &Self) -> String {
        return format!(
            "id: \"{}\"\ndisplay-name: \"{}\"\n---\n",
            self.id, self.display_name
        );
    }
}

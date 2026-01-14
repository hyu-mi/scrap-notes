use uuid::Uuid;

pub struct FolderMetadata {
    id: Uuid,
    display_name: String,
}

impl FolderMetadata {
    pub fn new(id: Uuid, display_name: impl Into<String>) -> Self {
        return Self {
            id,
            display_name: display_name.into(),
        };
    }

    pub fn get_id(self: &Self) -> Uuid {
        return self.id.clone();
    }

    pub fn get_name(self: &Self) -> String {
        return self.display_name.clone();
    }

    pub fn compose(self: &Self) -> String {
        return format!(
            "id: \"{}\"\ndisplay-name: \"{}\"\n---\n",
            self.id, self.display_name
        );
    }
}

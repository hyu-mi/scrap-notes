use uuid::Uuid;

pub enum Command {
    CreateNote {
        parent: Uuid,
        title: String,
        file_type: String,
    },
    CreateFolder {
        folder_id: Uuid,
        display_name: String,
    },
}

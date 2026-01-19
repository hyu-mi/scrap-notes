use uuid::Uuid;

#[derive(Debug)]
pub enum ScrapCommand {
    // Workspace related
    SyncWorkspace,
    ListFolders,
    ListNotes,
    // FindNotes {
    //     title: Option<String>,
    //     file_type: Option<String>,
    // },
    // FindFolders {
    //     display_name: String,
    // },

    // Creating things
    CreateNote {
        parent_id: Uuid,
        title: String,
        file_type: String,
    },
    CreateFolder {
        parent_id: Uuid,
        display_name: String,
    },
}

use crate::cli::CliCommand;

use scrap::{FolderSummary, NoteSummary, Scrap, ScrapCommand, ScrapResponse};
use std::{collections::HashMap, path::PathBuf, str::FromStr};
use uuid::{Uuid, uuid};

pub struct App {
    scrap: Scrap,
    workspace_id: Uuid,

    notes: HashMap<Uuid, NoteSummary>,
    title_index: HashMap<String, Vec<Uuid>>,
    file_type_index: HashMap<String, Vec<Uuid>>,

    folders: HashMap<Uuid, FolderSummary>,
    display_name_index: HashMap<Vec<String>, Uuid>,

    note_shorthand_ids: HashMap<String, Vec<Uuid>>,
    folder_shorthand_ids: HashMap<[u8; 6], Vec<Uuid>>,
}

impl App {
    pub fn new(workspace_dir: PathBuf) -> Self {
        return Self {
            scrap: Scrap::new(workspace_dir),
            workspace_id: uuid!("3e206920-6c75-7620-7520-6d722063656f"),

            notes: HashMap::new(),
            title_index: HashMap::new(),
            file_type_index: HashMap::new(),

            folders: HashMap::new(),
            display_name_index: HashMap::new(),

            note_shorthand_ids: HashMap::new(),
            folder_shorthand_ids: HashMap::new(),
        };
    }

    // pub fn init(self: &Self) -> Result<(), Scrap> {}

    pub fn execute(self: &mut Self, command: CliCommand) {
        match self.scrap.handle_command(ScrapCommand::SyncWorkspace) {
            ScrapResponse::WorkspaceSynced => {}
            ScrapResponse::Error(err) => println!("Failed to sync workspace directory. Error: {:?}", err),
            _ => {}
        }

        match self.scrap.handle_command(ScrapCommand::ListFolders) {
            ScrapResponse::FolderList(folders) => {
                for folder in folders {
                    let mut val = Vec::new();
                    val.push(folder.id);

                    let mut shorthand = [0u8; 6];
                    shorthand.copy_from_slice(&folder.id.to_string().as_bytes()[..6]);

                    self.folder_shorthand_ids.insert(shorthand, val);
                }
            }
            ScrapResponse::Error(err) => println!("Failed request folders. Error: {:?}", err),
            _ => {}
        }

        if let Some(response) = self.handle_command(command) {
            match response {
                ScrapResponse::NoteCreated(id) => println!("[Scrap-core] Note created with ID: {}", id),
                ScrapResponse::FolderCreated(id) => println!("[Scrap-core] Folder created with ID: {}", id),
                ScrapResponse::InvalidCommand => eprintln!("[Scrap-core] invalid command!"),
                ScrapResponse::Error(err) => eprintln!("[Scrap-core] Error: {:?}", err),
                _ => {}
            }
        }
    }

    fn handle_command(self: &mut Self, command: CliCommand) -> Option<ScrapResponse> {
        match command {
            CliCommand::Note {
                title,
                file_type,
                parent_id,
            } => {
                let ids = self.resolve_id(&parent_id);

                if ids.len() == 0 {
                    println!("[CLI] No folder found for id {}.", parent_id);
                    return None;
                }

                if ids.len() == 1 {
                    return Some(self.scrap.handle_command(scrap::ScrapCommand::CreateNote {
                        parent_id: ids[0],
                        title,
                        file_type,
                    }));
                }

                println!("More than one folders founder for id {}:", parent_id);
                for id in ids {
                    println!("{}", id);
                }
                return None;
            }

            CliCommand::Folder {
                display_name,
                parent_id,
            } => {
                let ids = self.resolve_id(&parent_id);

                if ids.len() == 1 {
                    return Some(self.scrap.handle_command(scrap::ScrapCommand::CreateFolder {
                        parent_id: ids[0],
                        display_name,
                    }));
                } else {
                    println!("More than one ids found:");
                    for id in ids {
                        println!("{}", id);
                    }
                    return None;
                }
            }
        }
    }

    fn resolve_id(self: &Self, input: &str) -> Vec<Uuid> {
        const SHORTHAND_SIZE: usize = 6;

        // Found full UUID
        if let Ok(id) = Uuid::parse_str(input) {
            return vec![id];
        }

        if input.len() == SHORTHAND_SIZE {
            let mut shorthand = [0u8; 6];
            shorthand.copy_from_slice(input.as_bytes());

            if let Some(ids) = self.folder_shorthand_ids.get(&shorthand) {
                return ids.clone();
            }
        }

        return Vec::new();
    }
}

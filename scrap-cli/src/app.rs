use crate::app_error::AppError;
use crate::cli::CliCommand;

use scrap::api::{FolderSummary, NoteSummary};
use scrap::{Scrap, ScrapCommand, ScrapError, ScrapEvent};
use std::io::Read;
use std::process::id;
use std::{collections::HashMap, path::PathBuf, str::FromStr};
use uuid::{Uuid, uuid};

pub struct App {
    scrap: Scrap,
    workspace_id: Uuid,

    notes: HashMap<Uuid, NoteSummary>,
    folders: HashMap<Uuid, FolderSummary>,
    note_shorthands: HashMap<[u8; 6], Vec<Uuid>>,
    folder_shorthands: HashMap<[u8; 6], Vec<Uuid>>,
}

impl App {
    pub fn new(workspace_dir: PathBuf) -> Self {
        return Self {
            scrap: Scrap::new(workspace_dir),
            workspace_id: uuid!("3e206920-6c75-7620-7520-6d722063656f"),

            notes: HashMap::new(),
            folders: HashMap::new(),
            note_shorthands: HashMap::new(),
            folder_shorthands: HashMap::new(),
        };
    }

    pub fn init(self: &mut Self) -> Result<(), AppError> {
        // Sync workspace
        if let Err(err) = self.scrap.handle_command(ScrapCommand::SyncWorkspace) {
            return Err(AppError::WorkspaceSyncFailed(format!("{:?}", err)));
        }

        // Update memory
        match self.scrap.handle_command(ScrapCommand::ListNotes) {
            Ok(event) => match event {
                ScrapEvent::NoteList(summaries) => {
                    for summary in summaries {
                        self.insert_note(summary);
                    }
                }

                _ => {
                    return Err(AppError::Invalid(format!(
                        "Return type not expected for command ListNotes."
                    )));
                }
            },
            Err(err) => return Err(AppError::ListNotesFailed(format!("{:?}", err))),
        }

        match self.scrap.handle_command(ScrapCommand::ListFolders) {
            Ok(event) => match event {
                ScrapEvent::FolderList(summaries) => {
                    for summary in summaries {
                        // println!("{}", summary.id);
                        self.insert_folder(summary);
                    }
                }

                _ => {
                    return Err(AppError::Invalid(format!(
                        "Return type not expected for command ListFolders."
                    )));
                }
            },
            Err(err) => return Err(AppError::ListFoldersFailed(format!("{:?}", err))),
        }

        return Ok(());
    }

    pub fn execute(self: &mut Self, command: CliCommand) {
        match command {
            CliCommand::Open { note_id } => self.handle_open(note_id),

            CliCommand::Note {
                title,
                file_type,
                parent_id,
            } => self.handle_note(title, file_type, parent_id),

            CliCommand::Folder {
                display_name,
                parent_id,
            } => self.handle_folder(display_name, parent_id),

            _ => {}
        }
    }

    fn handle_open(self: &Self, note_id: String) {
        let ids = self.resolve_note_id(&note_id);

        // No Note found
        if ids.len() == 0 {
            eprintln!("Error: No note found matching '{}'.", note_id);
            return;
        }

        // Multiple Notes found
        if ids.len() > 1 {
            eprintln!("Ambiguous ID '{}'. Found {} notes:", note_id, ids.len());
            for id in ids {
                let name = self.notes.get(&id).map(|f| f.title.as_ref()).unwrap_or("unkown");

                eprintln!("  {}: {}", id, name);
            }

            eprintln!("Please use a full UUID to specify.");

            return;
        }

        if let Some(id) = ids.get(0) {
            println!("Pretend this is the content of the note({})!", id);
        }
    }

    fn handle_note(self: &mut Self, title: String, file_type: String, parent_id: String) {
        let ids = self.resolve_folder_id(&parent_id);

        // No Folder found
        if ids.len() == 0 {
            eprintln!("Error: No folder found matching '{}'.", parent_id);
            return;
        }

        // Multiple Folders found
        if ids.len() > 1 {
            eprintln!("Ambiguous ID '{}'. Found {} folders:", parent_id, ids.len());
            for id in ids {
                let name = self
                    .folders
                    .get(&id)
                    .map(|f| f.display_name.as_ref())
                    .unwrap_or("unkown");

                eprintln!("  {}: {}", id, name);
            }

            eprintln!("Please use a full UUID to specify.");

            return;
        }

        if let Some(id) = ids.get(0) {
            match self.scrap.handle_command(ScrapCommand::CreateNote {
                parent_id: *id,
                title: title.clone(),
                file_type,
            }) {
                Ok(event) => match event {
                    ScrapEvent::NoteCreated(id) => println!("Note '{}' created with id: {}", title, id),
                    _ => println!("Return type not expected for command CreateNote."),
                },
                Err(err) => println!("Failed to create note with error: {:?}", err),
            }
        }
    }

    fn handle_folder(self: &mut Self, display_name: String, parent_id: String) {
        let ids = self.resolve_folder_id(&parent_id);

        // No Folder found
        if ids.len() == 0 {
            eprintln!("Error: No folder found matching '{}'.", parent_id);
            return;
        }

        // Multiple Folders found
        if ids.len() > 1 {
            eprintln!("Ambiguous ID '{}'. Found {} folders:", parent_id, ids.len());
            for id in ids {
                let name = self
                    .folders
                    .get(&id)
                    .map(|f| f.display_name.as_ref())
                    .unwrap_or("unkown");

                eprintln!("  {}: {}", id, name);
            }

            eprintln!("Please use a full UUID to specify.");

            return;
        }

        if let Some(id) = ids.get(0) {
            match self.scrap.handle_command(ScrapCommand::CreateFolder {
                parent_id: *id,
                display_name: display_name.clone(),
            }) {
                Ok(event) => match event {
                    ScrapEvent::FolderCreated(id) => println!("Folder '{}' created with id: {}", display_name, id),
                    _ => println!("Return type not expected for command CreateFolder."),
                },

                Err(err) => println!("Failed to create note with error: {:?}", err),
            }
        }
    }

    fn insert_note(self: &mut Self, note: NoteSummary) {
        let id: Uuid = note.id;

        self.notes.insert(id, note);

        // Store shorthand
        let shorthand = Self::to_shorthand(id);
        self.note_shorthands.entry(shorthand).or_default().push(id);
    }

    fn insert_folder(self: &mut Self, folder: FolderSummary) {
        let id: Uuid = folder.id;

        self.folders.insert(id, folder);

        // Store shorthand
        let shorthand = Self::to_shorthand(id);
        self.folder_shorthands.entry(shorthand).or_default().push(id);
    }

    fn resolve_note_id(self: &Self, input: &str) -> Vec<Uuid> {
        const SHORTHAND_SIZE: usize = 6;

        // Found full UUID
        if let Ok(id) = Uuid::parse_str(input) {
            return vec![id];
        }

        if input.len() == SHORTHAND_SIZE {
            let mut shorthand = [0u8; 6];
            shorthand.copy_from_slice(input.as_bytes());

            if let Some(ids) = self.note_shorthands.get(&shorthand) {
                return ids.clone();
            }
        }

        return Vec::new();
    }

    fn resolve_folder_id(self: &Self, input: &str) -> Vec<Uuid> {
        const SHORTHAND_SIZE: usize = 6;

        // Found full UUID
        if let Ok(id) = Uuid::parse_str(input) {
            return vec![id];
        }

        if input.len() == SHORTHAND_SIZE {
            let mut shorthand = [0u8; 6];
            shorthand.copy_from_slice(input.as_bytes());

            if let Some(ids) = self.folder_shorthands.get(&shorthand) {
                return ids.clone();
            }
        }

        return Vec::new();
    }

    fn to_shorthand(id: Uuid) -> [u8; 6] {
        let mut out = [0u8; 6];
        out.copy_from_slice(&id.to_string().as_bytes()[..6]);
        return out;
    }
}

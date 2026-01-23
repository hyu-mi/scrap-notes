use crate::app_error::AppError;
use crate::cli::CliCommand;

use scrap::api::{FolderSummary, NoteSummary};
use scrap::{Scrap, ScrapError};
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
        self.scrap
            .sync_workspace()
            .map_err(|err| AppError::WorkspaceSyncFailed(format!("{:?}", err)))?;

        // Update memory
        match self.scrap.list_notes() {
            Ok(notes) => {
                for note in notes {
                    self.insert_note(note);
                }
            }
            Err(err) => return Err(AppError::ListNotesFailed(format!("{:?}", err))),
        }

        match self.scrap.list_folders() {
            Ok(folders) => {
                for folder in folders {
                    self.insert_folder(folder);
                }
            }
            Err(err) => return Err(AppError::ListFoldersFailed(format!("{:?}", err))),
        }

        return Ok(());
    }

    pub fn execute(self: &mut Self, command: CliCommand) {
        match command {
            CliCommand::Open { id } => self.handle_open(id),

            CliCommand::Add {
                title,
                file_type,
                parent,
            } => self.handle_add(title, file_type, parent),

            CliCommand::NewFolder { name, parent } => self.handle_new_folder(name, parent),

            _ => {}
        }
    }

    fn handle_open(self: &mut Self, id: String) {
        let ids = self.resolve_note_id(&id);

        // No Note found
        if ids.len() == 0 {
            eprintln!("Error: No note found matching '{}'.", id);
            return;
        }

        // Multiple Notes found
        if ids.len() > 1 {
            eprintln!("Ambiguous ID '{}'. Found {} notes:", id, ids.len());
            for id in ids {
                let name = self.notes.get(&id).map(|f| f.title.as_ref()).unwrap_or("unkown");

                eprintln!("  {}: {}", id, name);
            }

            eprintln!("Please use a full UUID to specify.");

            return;
        }

        // if let Some(id) = ids.get(0) {
        //     match self.scrap.handle_command(ScrapCommand::GetNoteBody(*id)) {
        //         Ok(event) => {
        //             //
        //             match event {
        //                 ScrapEvent::NoteBody(body) => print_note(body),
        //                 _ => {
        //                     // Unexpected returned event
        //                     print!("Ambiguous Error: Unmatched Scrap event OR Scrap error.")
        //                 }
        //             }
        //         }
        //         Err(ScrapError::NoteNotFound(err_id)) => {
        //             // Our ids are cached correctly
        //             print!("Internal Error: Cached id '{}' does not exists anymore.", id);
        //         }
        //         Err(err) => {
        //             // TODO: Handle scrap errors
        //         }
        //     }
        // }
    }

    fn handle_add(self: &mut Self, title: String, file_type: String, parent: String) {
        let ids = self.resolve_folder_id(&parent);

        // No Folder found
        if ids.len() == 0 {
            eprintln!("Error: No folder found matching '{}'.", parent);
            return;
        }

        // Multiple Folders found
        if ids.len() > 1 {
            eprintln!("Ambiguous ID '{}'. Found {} folders:", parent, ids.len());
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

        let parent_id = ids.get(0).unwrap();
        match self.scrap.create_note(*parent_id, title.clone(), file_type) {
            Ok(note_id) => println!("Note '{}' created with id: {}", title, note_id),
            Err(err) => println!("Failed to create note with error: {:?}", err),
        }
    }

    fn handle_new_folder(self: &mut Self, display_name: String, parent: String) {
        let ids = self.resolve_folder_id(&parent);

        // No Folder found
        if ids.len() == 0 {
            eprintln!("Error: No folder found matching '{}'.", parent);
            return;
        }

        // Multiple Folders found
        if ids.len() > 1 {
            eprintln!("Ambiguous ID '{}'. Found {} folders:", parent, ids.len());
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

        let parent_id = ids.get(0).unwrap();
        match self.scrap.create_folder(*parent_id, display_name.clone()) {
            Ok(folder_id) => println!("Folder '{}' created with id: {}", display_name, folder_id),
            Err(err) => println!("Failed to create note with error: {:?}", err),
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

fn print_note(body: String) {
    // Clanker made code ahead! 🤖

    // let title = self.metadata.get_title();
    // let file_type = self.metadata.get_file_type();
    // let id = self.metadata.get_id().to_string();

    let cyan = "\x1b[38;5;213m";
    let gray = "\x1b[90m";
    let bold = "\x1b[1m";
    let reset = "\x1b[0m";

    let width = 60;
    let horiz = "─".repeat(width);

    println!("{gray}╭{}╮{reset}", horiz);

    let title_line = format!("{:<width$}", "Title", width = width - 5);
    println!("{gray}│{reset} 📝 {cyan}{bold}{}{reset} {gray}│{reset}", title_line);

    println!("{gray}├{}┤{reset}", horiz);

    // // Front matter
    // let id_line = format!("{:<width$}", id, width = width - 8);
    // println!("{gray}│{reset} {gray}ID:   {reset}{} {gray}│{reset}", id_line);

    // let type_line = format!("{:<width$}", file_type, width = width - 8);
    // println!(
    //     "{gray}│{reset} {gray}TYPE: {reset}{bold}{}{reset} {gray}│{reset}",
    //     type_line
    // );

    // println!("{gray}├{}┤{reset}", horiz);

    // Body
    for line in body.lines() {
        let content_line = format!("{:<width$}", line, width = width - 2);
        println!("{gray}│{reset} {} {gray}│{reset}", content_line);
    }

    println!("{gray}╰{}╯{reset}", horiz);
}

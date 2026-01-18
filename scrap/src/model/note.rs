use crate::model::NoteData;
use crate::model::NoteMetadata;

use std::path::PathBuf;
use uuid::Uuid;

pub struct Note {
    relative_path: PathBuf,
    metadata: NoteMetadata,
    body: String,
    is_dirty: bool,
    // TODO: Need a last modified var so we can sync without opening the file...
}

impl Note {
    pub fn new(relative_path: PathBuf, metadata: NoteMetadata) -> Self {
        return Self {
            relative_path,
            metadata,
            body: String::new(),
            is_dirty: false,
        };
    }

    pub fn from_data(relative_path: PathBuf, data: NoteData) -> Self {
        let id = data.id.unwrap_or_else(Uuid::new_v4);

        let title = data.title.unwrap_or_else(|| {
            relative_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string()
        });

        // TODO: resolve file type by detecting it
        let file_type = data.file_type.unwrap_or("rich-text".to_string());

        let metadata = NoteMetadata::new(id, title, file_type);

        return Self {
            relative_path,
            metadata,
            body: data.body,
            is_dirty: false,
        };
    }

    pub fn get_relative_path(self: &Self) -> PathBuf {
        return self.relative_path.clone();
    }

    pub fn get_id(self: &Self) -> Uuid {
        return self.metadata.get_id();
    }

    pub fn get_title(self: &Self) -> &str {
        return self.metadata.get_title();
    }

    pub fn get_file_type(self: &Self) -> &str {
        return self.metadata.get_file_type();
    }

    pub fn compose(self: &Self) -> String {
        let mut out = self.metadata.compose();
        out.push_str(&self.body);
        return out;
    }

    pub fn write_all(self: &mut Self, content: &str) {
        self.body = content.to_string();
    }

    pub fn print(&self) {
        // Clanker made code ahead! 🤖

        let title = self.metadata.get_title();
        let file_type = self.metadata.get_file_type();
        let id = self.metadata.get_id().to_string();

        let cyan = "\x1b[38;5;213m";
        let gray = "\x1b[90m";
        let bold = "\x1b[1m";
        let reset = "\x1b[0m";

        let width = 60;
        let horiz = "─".repeat(width);

        println!("{gray}╭{}╮{reset}", horiz);

        let title_line = format!("{:<width$}", title, width = width - 5);
        println!("{gray}│{reset} 📝 {cyan}{bold}{}{reset} {gray}│{reset}", title_line);

        println!("{gray}├{}┤{reset}", horiz);

        // Front matter
        let id_line = format!("{:<width$}", id, width = width - 8);
        println!("{gray}│{reset} {gray}ID:   {reset}{} {gray}│{reset}", id_line);

        let type_line = format!("{:<width$}", file_type, width = width - 8);
        println!(
            "{gray}│{reset} {gray}TYPE: {reset}{bold}{}{reset} {gray}│{reset}",
            type_line
        );

        println!("{gray}├{}┤{reset}", horiz);

        // Body
        for line in self.body.lines() {
            let content_line = format!("{:<width$}", line, width = width - 2);
            println!("{gray}│{reset} {} {gray}│{reset}", content_line);
        }

        println!("{gray}╰{}╯{reset}", horiz);
    }
}

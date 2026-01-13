use crate::core::model::note::Note;
use crate::core::model::note_metadata::NoteMetadata;
use std::fs::File;
use std::fs::Metadata;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use uuid::Uuid;

pub fn parse_file_to_note(
    file: &mut File,
    path: impl AsRef<Path>,
) -> Option<Note> {
    let mut file_content = String::new();
    file.seek(SeekFrom::Start(0)).ok()?;
    file.read_to_string(&mut file_content).ok()?;

    let mut id: Option<Uuid> = None;
    let mut display_name: Option<String> = None;
    let mut file_type: Option<String> = None;

    let mut offset = 0;
    let mut delimiter_found = false;
    for line in file_content.lines() {
        let line = line.trim();

        // Empty lines are ignored
        if line.is_empty() {
            continue;
        }

        offset += line.len() + 1;

        // Reached ending delimiter
        if line == "---" {
            delimiter_found = true;
            break;
        }

        // Maybe this is a metadata ?
        if let Some((key, value)) = line.split_once(':') {
            if let Some(extracted) = extract_quoted(value) {
                // Empty values are invalid
                if extracted.is_empty() {
                    continue;
                }

                match key.trim() {
                    "id" => id = Some(Uuid::from_str(extracted).ok()?),
                    "display-name" => {
                        display_name = Some(extracted.to_string())
                    }
                    "type" => file_type = Some(extracted.to_string()),
                    _ => {}
                }
            }
        }
    }

    // TODO: Handle this
    if !delimiter_found {
        return None;
    }

    let (id, display_name, file_type) = (id?, display_name?, file_type?);

    let content = file_content[offset..].to_string();
    let metadata = NoteMetadata::new(Uuid::new_v4(), &display_name, &file_type);
    return Some(Note::new(metadata, PathBuf::from(path.as_ref())));
}

fn extract_quoted(s: &str) -> Option<&str> {
    let start = s.find('"')?;
    let end = s.rfind('"')?;
    if start < end {
        Some(&s[start + 1..end])
    } else {
        None
    }
}

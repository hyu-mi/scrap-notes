use crate::core::text::extract_quoted::extract_quoted;
use std::str::FromStr;
use uuid::Uuid;

pub struct NoteData {
    pub id: Option<Uuid>,
    pub title: Option<String>,
    pub file_type: Option<String>,
    pub body: String,
}

impl NoteData {
    pub fn new() -> Self {
        return Self {
            id: None,
            title: None,
            file_type: None,
            body: String::new(),
        };
    }
}

pub fn parse_note(input: String) -> NoteData {
    let mut out_data = NoteData::new();

    let first_line = input.lines().next().unwrap_or("").trim();

    let mut metadata_end_line_found = false;
    let mut metadata_end_byte_offset = 0;

    // Find the closing delimiter
    for (index, line) in input.lines().enumerate() {
        let trimmed = line.trim();

        // Skip the very first line if the starting delimiter exists
        if index == 0 && first_line == "---" {
            continue;
        }

        if trimmed == "---" {
            metadata_end_line_found = true;

            if let Some(offset) = input.find(line) {
                metadata_end_byte_offset = offset + line.len();
            }
            break;
        }
    }

    // Closing delimiter not found, return the whole input as body
    if !metadata_end_line_found {
        out_data.body = input.to_string();
        return out_data;
    }

    let metadata_part = &input[..metadata_end_byte_offset];
    let body_part = &input[metadata_end_byte_offset..];

    for line in metadata_part.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed == "---" {
            continue;
        }

        if let Some((key, value)) = trimmed.split_once(':') {
            let Some(extracted_value) = extract_quoted(value) else {
                // No value found inside qouted, invalid
                continue;
            };

            // Empty values are invalid
            if extracted_value.is_empty() {
                continue;
            }

            match key.trim() {
                "id" => {
                    if let Ok(id) = Uuid::from_str(extracted_value) {
                        out_data.id = Some(id);
                    }
                }
                "title" => out_data.title = Some(extracted_value.to_string()),
                "type" => out_data.file_type = Some(extracted_value.to_string()),
                _ => {}
            }
        }
    }

    out_data.body = body_part.to_string();

    return out_data;
}

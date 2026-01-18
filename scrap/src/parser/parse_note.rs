use crate::model::{Note, NoteData};
use crate::text::extract_quoted::extract_quoted;

use std::str::FromStr;
use uuid::Uuid;

pub fn parse_note(input: String) -> NoteData {
    let mut out_data = NoteData::new();

    let mut opening_delimiter_found = false;
    let mut closing_delimiter_found = false;

    let mut metadata_end_byte_offset = 0;
    let mut current_byte_offset = 0;

    for (index, line) in input.lines().enumerate() {
        let trimmed = line.trim();
        current_byte_offset += line.len() + 1;

        // Skip opening delimiter
        if index == 0 && trimmed == "---" {
            opening_delimiter_found = true;
            continue;
        }

        // Found closing delimiter
        if trimmed == "---" {
            closing_delimiter_found = true;

            metadata_end_byte_offset = current_byte_offset;
            break;
        }
    }

    // No opening or closing delimiter not found, return the whole input as body
    if !opening_delimiter_found || !closing_delimiter_found {
        out_data.body = input;
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

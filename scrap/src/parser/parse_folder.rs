use crate::model::{Folder, FolderData};
use crate::text::extract_quoted::extract_quoted;

use std::str::FromStr;
use uuid::Uuid;

pub fn parse_folder(input: String) -> FolderData {
    let mut out_data = FolderData::new();

    for line in input.lines() {
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
                "display-name" => out_data.display_name = Some(extracted_value.to_string()),
                _ => {}
            }
        }
    }

    return out_data;
}

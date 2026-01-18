use crate::text::slugify::slugify;

pub fn sanitize_name(input: &str, max_name_length: usize) -> String {
    let base_slug = slugify(input);

    let safe_slug = if base_slug.is_empty() {
        "untitled".to_string()
    } else {
        base_slug
    };

    let output = if safe_slug.len() > max_name_length {
        safe_slug.chars().take(max_name_length).collect::<String>()
    } else {
        safe_slug
    };

    return output;
}

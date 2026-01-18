pub fn slugify(name: &str) -> String {
    let mut prev_dash = false;
    return name
        .chars()
        .filter_map(|c| match c {
            'a'..='z' | '0'..='9' => {
                prev_dash = false;
                Some(c)
            }
            'A'..='Z' => {
                prev_dash = false;
                Some(c.to_ascii_lowercase())
            }
            ' ' | '_' if !prev_dash => {
                prev_dash = true;
                Some('-')
            }
            '-' if !prev_dash => {
                prev_dash = true;
                Some(c)
            }
            _ => None,
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
}

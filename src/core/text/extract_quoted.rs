pub fn extract_quoted(s: &str) -> Option<&str> {
    let start = s.find('"')?;
    let end = s.rfind('"')?;
    if start < end {
        Some(&s[start + 1..end])
    } else {
        None
    }
}

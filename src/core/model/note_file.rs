use std::path::PathBuf;

pub struct NoteFile {
    pub name: String,
    pub relative_path: PathBuf,
    pub absolute_path: PathBuf,
}

impl NoteFile {
    pub fn new(name: impl Into<String>, relative_path: PathBuf, absolute_path: PathBuf) -> Self {
        return Self {
            name: name.into(),
            relative_path,
            absolute_path,
        };
    }
}

use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn create_file(path: impl AsRef<Path>) -> io::Result<File> {
    return OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path.as_ref());
}

pub fn write_file(path: impl AsRef<Path>, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(false)
        .truncate(true)
        .open(path.as_ref())?;

    file.write_all(content.as_bytes())?;
    file.sync_all()?;

    return Ok(());
}

pub fn open_file(path: impl AsRef<Path>) -> io::Result<File> {
    return OpenOptions::new()
        .read(true)
        .write(true)
        .open(path.as_ref());
}

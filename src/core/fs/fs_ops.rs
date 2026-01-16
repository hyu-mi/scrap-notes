use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs::ReadDir;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub fn open_file(workspace_dir: &Path, target_dir: &Path) -> io::Result<File> {
    let target = resolve_existing_path(workspace_dir, target_dir)?;

    return OpenOptions::new().read(true).write(true).create(false).open(&target);
}

pub fn create_file(workspace_dir: &Path, target_dir: &Path) -> io::Result<File> {
    let target = resolve_new_path(workspace_dir, target_dir)?;

    return OpenOptions::new().write(true).create_new(true).open(&target);
}

pub fn delete_file(workspace_dir: &Path, target_dir: &Path) -> io::Result<()> {
    let target = resolve_existing_path(workspace_dir, target_dir)?;

    fs::remove_file(&target)?;

    return Ok(());
}

pub fn write_file(workspace_dir: &Path, target_dir: &Path, content: &str) -> io::Result<()> {
    let target = resolve_existing_path(workspace_dir, target_dir)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(false)
        .truncate(true)
        .open(&target)?;

    file.write_all(content.as_bytes())?;

    file.sync_all()?;

    return Ok(());
}

pub fn create_dir(workspace_dir: &Path, target_dir: &Path) -> io::Result<()> {
    let target = resolve_new_dir(workspace_dir, target_dir)?;

    return fs::create_dir(&target);
}

pub fn delete_dir(workspace_dir: &Path, target_dir: &Path) -> io::Result<()> {
    let target = resolve_existing_dir(workspace_dir, target_dir)?;

    return fs::remove_dir(&target);
}

pub fn read_directory(workspace_dir: &Path, target_dir: &Path) -> io::Result<ReadDir> {
    let target_dir = resolve_existing_dir(workspace_dir, target_dir)?;

    return fs::read_dir(target_dir);
}

fn resolve_new_path(workspace_dir: &Path, target_dir: &Path) -> io::Result<PathBuf> {
    let target = workspace_dir.join(target_dir);

    let parent_dir = target
        .parent()
        .ok_or(io::Error::new(io::ErrorKind::NotADirectory, ""))?;

    // Reject any invalid path, path traversal, or symbolic injection
    let canonical_parent_dir = parent_dir.canonicalize()?;

    // Reject target paths that are outside workspace
    if !canonical_parent_dir.starts_with(&workspace_dir) {
        return Err(io::Error::new(io::ErrorKind::NotADirectory, ""));
    }

    return Ok(target);
}

fn resolve_existing_path(workspace_dir: &Path, target_dir: &Path) -> io::Result<PathBuf> {
    let target: PathBuf = workspace_dir.join(target_dir);

    // Reject any invalid path, path traversal, or symbolic injection
    let canonical_target = target.canonicalize()?;

    // Reject target paths that are outside workspace
    if !canonical_target.starts_with(&workspace_dir) {
        return Err(io::Error::new(io::ErrorKind::NotADirectory, ""));
    }

    return Ok(canonical_target);
}

fn resolve_new_dir(workspace_dir: &Path, target_dir: &Path) -> io::Result<PathBuf> {
    let target = workspace_dir.join(target_dir);

    let parent_dir = target.parent().ok_or(io::Error::new(io::ErrorKind::InvalidInput, ""))?;

    // Reject any invalid path, path traversal, or symbolic injection
    let canonical_parent_dir = parent_dir.canonicalize()?;

    // Reject target paths that are outside workspace
    if !canonical_parent_dir.starts_with(&workspace_dir) {
        return Err(io::Error::new(io::ErrorKind::NotADirectory, ""));
    }

    return Ok(target);
}

fn resolve_existing_dir(workspace_dir: &Path, target_dir: &Path) -> io::Result<PathBuf> {
    let target = workspace_dir.join(target_dir);

    // Reject any invalid path, path traversal, or symbolic injection
    let canonical_target = target.canonicalize()?;

    // Reject target paths that are outside workspace
    if !canonical_target.starts_with(&workspace_dir) {
        return Err(io::Error::new(io::ErrorKind::NotADirectory, ""));
    }

    return Ok(target);
}

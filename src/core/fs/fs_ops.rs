use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use crate::core::fs::fs_error::FSError;
use crate::core::fs::fs_event::FSEvent;

pub fn create_file(workspace_dir: &Path, relative_path: &Path) -> Result<File, FSError> {
    let target = resolve_path(workspace_dir, relative_path)?;

    return OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&target)
        .map_err(FSError::from_io);
}

pub fn write_file(
    workspace_dir: &Path,
    relative_path: &Path,
    content: &str,
) -> Result<FSEvent, FSError> {
    let target = resolve_path(workspace_dir, relative_path)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(false)
        .truncate(true)
        .open(&target)
        .map_err(FSError::from_io)?;

    file.write_all(content.as_bytes())
        .map_err(FSError::from_io)?;
    file.sync_all().map_err(FSError::from_io)?;

    return Ok(FSEvent::FileSaved);
}

pub fn open_file(workspace_dir: &Path, relative_path: &Path) -> Result<File, FSError> {
    let target = resolve_path(workspace_dir, relative_path)?;

    return OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(&target)
        .map_err(FSError::from_io);
}

// pub fn create_dir(workspace_dir: &Path, relative_path: &Path) -> io::Result<File> {
//     let target = resolve_path(workspace_dir, relative_path);

//     return fs_ops::create_file(&target);
// }

fn resolve_path(workspace_dir: &Path, relative_path: &Path) -> Result<PathBuf, FSError> {
    if relative_path.is_absolute() {
        return Err(FSError::PermissionDenied(
            "Absolute paths not allowed".to_string(),
        ));
    }

    for component in relative_path.components() {
        match component {
            Component::Normal(_) | Component::CurDir => continue,
            _ => return Err(FSError::PermissionDenied("Path traversal detected".into())),
        }
    }

    let target = workspace_dir.join(relative_path);
    if !target.starts_with(workspace_dir) {
        return Err(FSError::PermissionDenied(
            "Target path outside workspace directory".to_string(),
        ));
    }

    return Ok(target);
}

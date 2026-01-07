use std::path::PathBuf;

pub struct Config {
    pub workspace_dir: PathBuf,
}

impl Config {
    pub fn load() -> Self {
        let default_path: PathBuf = std::env::current_dir()
            .expect("Failed to get current directory")
            .join(".workspace");

        std::fs::create_dir_all(&default_path).expect("Failed to create workspace folder");

        Config {
            workspace_dir: default_path,
        }
    }
}

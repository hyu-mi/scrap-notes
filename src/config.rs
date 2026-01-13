use std::path::PathBuf;

pub struct Config {
    pub root: PathBuf,
}

impl Config {
    pub fn load() -> Self {
        let path = std::env::current_dir().expect("Failed to get current directory").join(".workspace");
        std::fs::create_dir_all(&path).expect("Failed to create workspace folder");
        return Self { root: path };
    }
}

use crate::{Config, core::core::Core};

pub struct App {
    core: Core,
}

impl App {
    pub fn new(config: Config) -> Self {
        let dir = config.root_directory.clone();
        App { core: Core::new(dir) }
    }

    pub fn run(&mut self) {
        let id = self.core.create_text("bog").expect("msg");
        self.core
            .write_file_content(id, String::from("This content was created at 15 line of app.rs!"));

        // Auto save trigger !
        self.core.auto_save();
    }

    // pub fn handle_command(self: &mut Self, command: AppCommand) -> Result<AppEvent, AppError> {
    //     match command {
    //         AppCommand::CreateFile { name, file_type } => {
    //             let result = self.create_text(&name);
    //             match result {
    //                 Ok(val) => return Ok(AppEvent::FileCreated { name: val }),
    //                 _ => return Err(AppError::IoError),
    //             }
    //         }
    //         AppCommand::OpenFile { name } => {
    //             return Ok(AppEvent::FileOpened {
    //                 content: String::from(""),
    //             });
    //         }
    //         AppCommand::SaveFile { name, content } => {
    //             return Ok(AppEvent::FileSaved);
    //         }
    //     }
    // }

    // fn create_text(self: &Self, name: &str) -> std::io::Result<PathBuf> {
    //     let result = FileManager::create_file(&self.config.workspace_dir, name, "txt");
    //     match result {
    //         Ok(file) => return Ok(PathBuf::from(file)),
    //         Err(e) => Err(e),
    //     }
    // }

    // fn save_text(self: &Self, path: &PathBuf, content: &str) {
    //     let result = FileManager::save_file(path, content);
    // }
}

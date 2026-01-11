use std::str::FromStr;

use uuid::Uuid;

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
        // Load content from root directory
        self.core.load_content();

        // ~ test creating and writing files
        let id = self
            .core
            .create_text("  bog  __--- engine:  ")
            .expect("failed to create a file!");
        self.core
            .write_all(&id, String::from("This content was created in app.rs!"));

        // self.core.create_folder("Hello").expect("failed to create a folder!");

        // ~ test modify existing file
        let existing_id = Uuid::from_str("e80651a4-c1d3-42e6-8962-82a8005ca9cc").ok().unwrap();
        self.core.write_all(
            &existing_id,
            String::from("cause I can feel a real connection, a supernatural attraction-ah~!"),
        );

        // Auto save trigger !
        self.core.save_all_dirty_files();
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
}

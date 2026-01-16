use crate::core::core::Core;
use crate::core::core_error;
use crate::core::core_event;
use crate::core::core_event::CoreEvent;
use crate::core::workspace;

pub struct App {
    core: Core,
}

impl App {
    pub fn new() -> Self {
        let path = std::env::current_dir()
            .expect("Failed to get current directory")
            .join(".workspace");
        std::fs::create_dir_all(&path).expect("Failed to create workspace folder");

        let root = path.canonicalize().expect("Could not canonicalize root path");

        return Self { core: Core::new(root) };
    }

    pub fn run(self: &mut Self) {
        self.core.load_content();

        let workspace_id = self.core.get_workspace_id();

        self.core
            .create_note(workspace_id, " 258 ", "rich-text")
            .expect("Failed to create note at root");

        let new_folder_id = self
            .core
            .create_folder(workspace_id, "XOXO")
            .expect("Failed to create folder at root")
            .created_id()
            .expect("NOPE!");

        let new_note_id = self
            .core
            .create_note(new_folder_id, "VIVIZ this love is maniac maniac maniac", "rich-text")
            .expect("Failed to create new note in a folder")
            .created_id()
            .expect("WHAT??");

        let name = "viviz-this-love-is-maniac-maniac-maniacdaipfjapdjfipheaifhaieghiodahhhhhhhhhhhhhhhhegphaeiUOhifjsgisfogipsfgjsiofpjgisfjgisfjgoiushfuorgjpiserjmgipspghrsipjgrsipgjsrigrjsigrsjigiprsgegfuioeahdaojkgiapoooooooooooooooooodhghdaioguashguoiadjfpiajkfopwehfuerjgoihrsipgfjdakngvujorjiytisouyfiuodahgHI!";
        self.core
            .create_note(workspace_id, name, "rich-text")
            .expect("Failed to create note with large name");
    }
}

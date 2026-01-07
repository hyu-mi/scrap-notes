pub enum AppCommand {
    CreateFile { name: String, file_type: String },
    OpenFile { name: String },
    SaveFile { name: String, content: String },
}

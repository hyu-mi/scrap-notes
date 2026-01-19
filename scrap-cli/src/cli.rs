use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    Open {
        note_id: String,
    },

    Note {
        title: String,
        #[arg(short, long, default_value = "plain-text")]
        file_type: String,
        #[arg(short, long, default_value = "3e206920-6c75-7620-7520-6d722063656f")]
        parent_id: String,
    },
    Folder {
        display_name: String,
        #[arg(short, long, default_value = "3e206920-6c75-7620-7520-6d722063656f")]
        parent_id: String,
    },
}

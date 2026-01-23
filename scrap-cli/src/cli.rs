use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    Open {
        id: String,
    },

    Add {
        title: String,
        #[arg(short, long, default_value = "plain-text")]
        file_type: String,
        #[arg(short, long, default_value = "3e206920-6c75-7620-7520-6d722063656f")]
        parent: String,
    },

    Remove {
        #[arg(value_enum)]
        kind: ItemKind,
        id: String,
    },

    NewFolder {
        name: String,
        #[arg(short, long, default_value = "3e206920-6c75-7620-7520-6d722063656f")]
        parent: String,
    },
}

#[derive(clap::ValueEnum, Clone)]
pub enum ItemKind {
    Note,
    Folder,
}

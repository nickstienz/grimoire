use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates a new project
    New(NewArgs),
}

#[derive(Parser)]
pub struct NewArgs {
    /// The name of the project
    pub name: String,
}

use clap::{Parser, Subcommand};

pub mod rust;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// new or initialize `Rust` project
    #[clap(subcommand)]
    Rust(self::rust::Args),
}

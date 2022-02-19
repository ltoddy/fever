use clap::{Parser, Subcommand};

pub mod haskell;
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

    /// new or initialize `Haskell` project
    #[clap(subcommand)]
    Haskell(self::haskell::Args),
}

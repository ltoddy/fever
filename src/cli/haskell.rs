use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Args {
    Init(InitArgs),
    New(NewArgs),
}

#[derive(Debug, Parser)]
pub struct InitArgs {
    #[clap(long = "name")]
    pub name: Option<String>,
}

#[derive(Debug, Parser)]
pub struct NewArgs {
    pub path: PathBuf,

    #[clap(long = "name")]
    pub name: Option<String>,
}

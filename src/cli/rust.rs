use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Error;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Args {
    /// Create a new Cargo package in an existing directory
    Init(InitArgs),
    /// Create a new Cargo package
    New(NewArgs),
}

// can't specify both lib and binary outputs

#[derive(Clone, Debug, Parser)]
pub struct InitArgs {
    #[clap(long = "kind", default_value = "bin")]
    pub kind: NewProjectKind,

    /// Set the resulting package name, defaults to the directory name
    #[clap(long = "name")]
    pub name: Option<String>,

    /// Edition to set for the crate generated [possible values: 2015, 2018, 2021]
    #[clap(long = "edition", default_value = "2021")]
    pub edition: String,

    #[clap(long = "description", default_value = "")]
    pub description: String,
}

#[derive(Clone, Debug, Parser)]
pub struct NewArgs {
    pub path: PathBuf,

    #[clap(long = "kind", default_value = "bin")]
    pub kind: NewProjectKind,

    /// Set the resulting package name, defaults to the directory name
    #[clap(long = "name")]
    pub name: Option<String>,

    /// Edition to set for the crate generated [possible values: 2015, 2018, 2021]
    #[clap(long = "edition", default_value = "2021")]
    pub edition: String,

    #[clap(long = "description", default_value = "")]
    pub description: String,
    // pub registry: Option<String>, TODO
}

#[derive(Clone, Debug, Subcommand)]
pub enum NewProjectKind {
    Bin,
    Lib,
}

impl FromStr for NewProjectKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bin" => Ok(NewProjectKind::Bin),
            "lib" => Ok(NewProjectKind::Lib),
            _ => unreachable!(),
        }
    }
}

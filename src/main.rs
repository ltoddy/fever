// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use log::LevelFilter::Debug;
use simplelog::{ColorChoice, TermLogger, TerminalMode};

use crate::cli::{Cli, SubCommands};

pub mod cli;
pub mod filesystem;
pub mod git;
pub mod haskell;
pub mod rust;
pub mod template;

pub fn main() -> Result<()> {
    TermLogger::init(Debug, Default::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    let cli: Cli = Cli::parse();
    let now = Instant::now();
    match cli.subcommand {
        SubCommands::Rust(args) => crate::rust::ProjectMaker::new().execute(args)?,
        SubCommands::Haskell(args) => crate::haskell::ProjectMaker::new().execute(args)?,
    }
    let elapsed = now.elapsed();
    log::info!("finished make project in {:?}", elapsed);
    Ok(())
}

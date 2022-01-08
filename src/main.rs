// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use anyhow::Result;
use log::LevelFilter::Debug;
use simplelog::{ColorChoice, TermLogger, TerminalMode};
use structopt::StructOpt;

use crate::options::Options;
use crate::rust::RustProjectMaker;

pub mod filesystem;
pub mod git;
pub mod macros;
pub mod options;
pub mod rust;
pub mod template;

pub fn main() -> Result<()> {
    TermLogger::init(Debug, Default::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    let opts: Options = Options::from_args();
    match opts {
        Options::Rust(options) => RustProjectMaker::new().execute(options)?,
    }
    Ok(())
}

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use anyhow::Result;
use structopt::StructOpt;

use crate::options::Options;

pub mod options;
pub mod rust;

pub fn main() -> Result<()> {
    let opts: Options = Options::from_args();
    match opts {
        Options::Rust(options) => crate::rust::create(options)?,
    }
    Ok(())
}

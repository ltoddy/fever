// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use anyhow::Result;

use crate::cli::haskell::Args;

#[derive(Default)]
pub struct ProjectMaker {}

impl ProjectMaker {
    pub fn new() -> Self {
        ProjectMaker {}
    }

    pub fn execute(self, args: Args) -> Result<()> {
        println!("args: {args:?}");
        Ok(())
    }
}

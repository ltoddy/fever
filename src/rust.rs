// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use anyhow::{Context, Result};

use crate::options::rust;

const RUST_CARGO_TOML: &str = include_str!("templates/rust/Cargo.toml");

pub fn create(options: rust::NewOptions) -> Result<()> {
    let project_directory = fs::canonicalize(&options.path)
        .with_context(|| format!("can't make `{}` canonically", options.path.display()))?;
    if !project_directory.is_dir() {
        fs::create_dir(&project_directory)
            .with_context(|| format!("can't create directory `{:?}`", project_directory.display()))?;
    }
    let src_directory = project_directory.join("src");
    if !src_directory.is_dir() {
        fs::create_dir(&src_directory)
            .with_context(|| format!("can't create directory `{:?}`", src_directory.display()))?;
    }

    Ok(())
}

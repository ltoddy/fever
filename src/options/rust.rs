// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Error;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
pub enum Options {
    Init(InitOptions),
    New(NewOptions),
}

#[derive(Clone, Debug, StructOpt)]
pub struct InitOptions {
    #[structopt(long = "kind", default_value = "bin")]
    pub kind: NewProjectKind,

    #[structopt(long = "name")]
    pub name: Option<String>,

    #[structopt(long = "edition", default_value = "2021")]
    pub edition: String,
}

#[derive(Clone, Debug, StructOpt)]
pub struct NewOptions {
    pub path: PathBuf,

    #[structopt(long = "kind", default_value = "bin")]
    pub kind: NewProjectKind,

    #[structopt(long = "name")]
    pub name: Option<String>,

    #[structopt(long = "edition", default_value = "2021")]
    pub edition: String,
    // pub registry: Option<String>, TODO
}

#[derive(Clone, Debug, StructOpt)]
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

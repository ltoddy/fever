// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use std::fs;
use std::ops::Deref;

use anyhow::{Context, Result};
use chrono::{Datelike, Local};
use git2::Repository;
use lazy_static::lazy_static;
use serde::Serialize;

use crate::git::RepositoryExt;
use crate::hash_map;
use crate::options::rust::{InitOptions, NewOptions, Options};
use crate::template::render;

lazy_static! {
    static ref FILES: HashMap<&'static str, &'static str> = hash_map! {
        ".license.template" => include_str!("templates/rust/.license.template"),
        ".gitignore" => include_str!("templates/rust/.gitignore"),
        "src/main.rs" => include_str!("templates/rust/src/main.rs"),
        "rustfmt.toml" => include_str!("templates/rust/rustfmt.toml"),
        "LICENSE-APACHE" => include_str!("templates/rust/LICENSE-APACHE"),
    };
    static ref TEMPLATES: HashMap<&'static str, &'static str> = hash_map! {
        "Cargo.toml" => include_str!("templates/rust/Cargo.toml.template"),
        "README.md"  => include_str!("templates/rust/README.md.template"),
        "LICENSE-MIT" => include_str!("templates/rust/LICENSE-MIT"),
    };
}

#[derive(Debug, Serialize)]
struct TemplateContext {
    username: String,
    email: String,
    project: String,
    edition: String,
    description: String,

    year: i32,
}

impl TemplateContext {
    pub fn new(username: String, email: String, project: String, edition: String, description: String) -> Self {
        let year = Local::now().year();

        TemplateContext { username, email, project, edition, description, year }
    }
}

// TODO
pub fn exec(options: Options) -> Result<()> {
    match options {
        Options::Init(options) => init(options),
        Options::New(options) => new(options),
    }
}

fn init(options: InitOptions) -> Result<()> {
    Ok(())
}

fn new(options: NewOptions) -> Result<()> {
    let NewOptions { path: project_dir, kind, name, edition, description } = options;

    if !project_dir.is_dir() {
        log::info!("creating directory `{}`", project_dir.display());
        fs::create_dir(&project_dir).with_context(|| format!("can't create directory `{:?}`", project_dir.display()))?;
    }
    let src_dir = project_dir.join("src");
    if !src_dir.is_dir() {
        log::info!("creating directory `{}`", src_dir.display());
        fs::create_dir(&src_dir).with_context(|| format!("can't create directory `{:?}`", src_dir.display()))?;
    }

    log::info!("initializing git repository for `{}`", project_dir.display());
    let repository = Repository::init(&project_dir).with_context(|| "initialize git repository failed")?;
    let email = repository.get_email();
    let username = repository.get_username();
    let project =
        name.unwrap_or_else(|| project_dir.file_stem().and_then(|name| name.to_str()).map(|name| name.to_string()).unwrap_or_default());
    let context = TemplateContext::new(username, email, project, edition, description);

    for (dst, content) in TEMPLATES.deref() {
        log::info!("writing file `{}`", dst);
        fs::write(project_dir.join(dst), render(content, &context)).with_context(|| format!("write `{}` failed", dst))?;
    }
    for (dst, content) in FILES.deref() {
        log::info!("writing file `{}`", dst);
        fs::write(project_dir.join(dst), content).with_context(|| format!("write `{}` failed", dst))?;
    }

    Ok(())
}

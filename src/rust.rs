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
use tinytemplate::TinyTemplate;

use crate::hash_map;
use crate::options::rust;

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
    name: String,
    email: String,
    project: String,
    year: i32,
}

impl TemplateContext {
    pub fn new(name: String, email: String, project: String, year: i32) -> Self {
        TemplateContext { name, email, project, year }
    }
}

fn render_template(content: &str, context: &TemplateContext) -> String {
    let mut template = TinyTemplate::new();
    template.add_template("just", content).and_then(|_| template.render("just", context)).unwrap()
}

pub fn create(options: rust::NewOptions) -> Result<()> {
    let now = Local::now();
    let project_directory = options.path;
    if !project_directory.is_dir() {
        fs::create_dir(&project_directory)
            .with_context(|| format!("can't create directory `{:?}`", project_directory.display()))?;
    }
    let src_directory = project_directory.join("src");
    if !src_directory.is_dir() {
        fs::create_dir(&src_directory)
            .with_context(|| format!("can't create directory `{:?}`", src_directory.display()))?;
    }

    let repository = Repository::init(&project_directory).with_context(|| "initialize git repository failed")?;
    let repo_config = repository.config()?;
    let email = repo_config.get_string("user.email").with_context(|| {
        "can't get `user.email`, make sure to execute to `git config --global user.email <email>` command"
    })?;
    let name = repo_config.get_string("user.name").with_context(|| {
        "can't get `user.email`, make sure to execute to `git config --global user.name <name>` command"
    })?;
    let project =
        project_directory.file_stem().and_then(|name| name.to_str()).map(|name| name.to_string()).unwrap_or_default();
    for (dst, content) in FILES.deref() {
        fs::write(project_directory.join(dst), content).with_context(|| format!("write `{}` failed", dst))?;
    }
    let context = TemplateContext::new(name, email, project, now.year());
    for (dst, content) in TEMPLATES.deref() {
        fs::write(project_directory.join(dst), render_template(content, &context))
            .with_context(|| format!("write `{}` failed", dst))?;
    }

    Ok(())
}

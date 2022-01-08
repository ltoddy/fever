// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{Datelike, Local};
use git2::Repository;
use serde::Serialize;

use crate::filesystem::basename;
use crate::git::RepositoryExt;
use crate::hash_map;
use crate::options::rust::{InitOptions, NewOptions, Options};
use crate::template::render;

#[derive(Debug, Serialize)]
struct TemplateContext {
    username: String,
    email: String,
    name: String,
    // project name
    edition: String,
    description: String,

    year: i32,
}

impl TemplateContext {
    pub fn new(username: String, email: String, name: String, edition: String, description: String) -> Self {
        let year = Local::now().year();

        TemplateContext { username, email, name, edition, description, year }
    }
}

#[derive(Default)]
pub struct RustProjectMaker {
    files: HashMap<PathBuf, &'static str>,
    templates: HashMap<PathBuf, &'static str>,
}

impl RustProjectMaker {
    pub fn new() -> Self {
        let files = hash_map! {
            PathBuf::from(".license.template") => include_str!("templates/rust/.license.template"),
            PathBuf::from(".gitignore") => include_str!("templates/rust/.gitignore"),
            PathBuf::from("src/main.rs") => include_str!("templates/rust/src/main.rs"),
            PathBuf::from("rustfmt.toml") => include_str!("templates/rust/rustfmt.toml"),
            PathBuf::from("LICENSE-APACHE") => include_str!("templates/rust/LICENSE-APACHE"),
        };
        let templates = hash_map! {
            PathBuf::from("Cargo.toml") => include_str!("templates/rust/Cargo.toml.template"),
            PathBuf::from("README.md")  => include_str!("templates/rust/README.md.template"),
            PathBuf::from("LICENSE-MIT") => include_str!("templates/rust/LICENSE-MIT"),
        };

        RustProjectMaker { files, templates }
    }

    pub fn execute(self, options: Options) -> Result<()> {
        match options {
            Options::Init(options) => self.initialize_project(options),
            Options::New(options) => self.new_project(options),
        }
    }

    fn initialize_project(self, options: InitOptions) -> Result<()> {
        let InitOptions { kind: _, name, edition, description } = options;
        let project_dir = current_dir().with_context(|| "the current working directory value is invalid")?;

        Self::create_src_directory(&project_dir)?;

        log::info!("initializing git repository for `{}`", project_dir.display());
        let repository = Repository::init(&project_dir).with_context(|| "initialize git repository failed")?;
        let user = repository.current_user();
        let name = name.or_else(|| basename(&project_dir)).unwrap_or_default();
        let context = TemplateContext::new(user.name, user.email, name, edition, description);

        self.create_project_files(&project_dir, &context)?;
        Ok(())
    }

    fn new_project(self, options: NewOptions) -> Result<()> {
        let NewOptions { path: project_dir, kind: _, name, edition, description } = options;

        if !project_dir.is_dir() {
            log::info!("creating directory `{}`", project_dir.display());
            fs::create_dir(&project_dir).with_context(|| format!("can't create directory `{:?}`", project_dir.display()))?;
        }
        Self::create_src_directory(&project_dir)?;

        log::info!("initializing git repository for `{}`", project_dir.display());
        let repository = Repository::init(&project_dir).with_context(|| "initialize git repository failed")?;
        let user = repository.current_user();
        let name = name.or_else(|| basename(&project_dir)).unwrap_or_default();
        let context = TemplateContext::new(user.name, user.email, name, edition, description);

        self.create_project_files(&project_dir, &context)?;
        Ok(())
    }

    fn create_project_files(self, project_dir: &Path, context: &TemplateContext) -> Result<()> {
        let Self { files, templates } = self;
        for (dst, content) in files {
            log::info!("writing file `{}`", dst.display());
            fs::write(project_dir.join(&dst), content).with_context(|| format!("write `{}` failed", dst.display()))?;
        }
        for (dst, content) in templates {
            log::info!("writing file `{}`", dst.display());
            fs::write(project_dir.join(&dst), render(content, context)).with_context(|| format!("write `{}` failed", dst.display()))?;
        }

        Ok(())
    }

    fn create_src_directory(project_dir: &Path) -> Result<PathBuf> {
        let src_dir = project_dir.join("src");
        if !src_dir.is_dir() {
            log::info!("creating directory `{}`", src_dir.display());
            fs::create_dir(&src_dir).with_context(|| format!("can't create directory `{:?}`", src_dir.display()))?;
        }
        Ok(src_dir)
    }
}

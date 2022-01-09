// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use git2::Repository;
use serde::Serialize;

use crate::datetime::current_year;
use crate::filesystem::basename;
use crate::git::RepositoryExt;
use crate::options::rust::{InitOptions, NewOptions, NewProjectKind, Options};
use crate::template::render;

const LICENSE_TEMPLATE: &str = include_str!("templates/rust/.license.template");
const GIT_IGNORE: &str = include_str!("templates/rust/.gitignore");
const MAIN_RS: &str = include_str!("templates/rust/src/main.rs");
const LIB_RS: &str = include_str!("templates/rust/src/lib.rs");
const RUSTFMT_TOML: &str = include_str!("templates/rust/rustfmt.toml");
const RUST_TOOLCHAIN: &str = include_str!("templates/rust/rust-toolchain");
const LICENSE_APACHE: &str = include_str!("templates/rust/LICENSE-APACHE");
const GITHUB_WORKFLOWS_CI_YML: &str = include_str!("templates/rust/.github/workflows/ci.yml");

const CARGO_TOML_TEMPLATE: &str = include_str!("templates/rust/Cargo.toml.template");
const README_TEMPLATE: &str = include_str!("templates/rust/README.md.template");
const LICENSE_MIT: &str = include_str!("templates/rust/LICENSE-MIT");

#[derive(Debug, Serialize)]
struct TemplateContext {
    username: String,
    email: String,
    // project name
    name: String,
    edition: String,
    description: String,

    year: i32,
}

impl TemplateContext {
    pub fn new(username: String, email: String, name: String, edition: String, description: String) -> Self {
        let year = current_year();

        TemplateContext { username, email, name, edition, description, year }
    }
}

#[derive(Default)]
pub struct RustProjectMaker {
    bin_file: (PathBuf, &'static str),
    lib_file: (PathBuf, &'static str),
    common_plain_files: Vec<(PathBuf, &'static str)>,
    templates: Vec<(PathBuf, &'static str)>,
}

impl RustProjectMaker {
    pub fn new() -> Self {
        let bin_file = (PathBuf::from("src/main.rs"), MAIN_RS);
        let lib_file = (PathBuf::from("src/lib.rs"), LIB_RS);
        let common_plain_files = vec![
            (PathBuf::from(".license.template"), LICENSE_TEMPLATE),
            (PathBuf::from(".gitignore"), GIT_IGNORE),
            (PathBuf::from("rustfmt.toml"), RUSTFMT_TOML),
            (PathBuf::from("rust-toolchain"), RUST_TOOLCHAIN),
            (PathBuf::from("LICENSE-APACHE"), LICENSE_APACHE),
            (PathBuf::from(".github/workflows/ci.yml"), GITHUB_WORKFLOWS_CI_YML),
        ];
        let templates = vec![
            (PathBuf::from("Cargo.toml"), CARGO_TOML_TEMPLATE),
            (PathBuf::from("README.md"), README_TEMPLATE),
            (PathBuf::from("LICENSE-MIT"), LICENSE_MIT),
        ];

        RustProjectMaker { bin_file, lib_file, common_plain_files, templates }
    }

    pub fn execute(self, options: Options) -> Result<()> {
        match options {
            Options::Init(options) => self.initialize_project(options),
            Options::New(options) => self.new_project(options),
        }
    }

    fn initialize_project(self, options: InitOptions) -> Result<()> {
        let InitOptions { kind, name, edition, description } = options;
        let project_dir = current_dir().with_context(|| "the current working directory value is invalid")?;

        Self::create_github_workflows_directory(&project_dir)?;
        Self::create_src_directory(&project_dir)?;

        log::info!("initializing git repository for `{}`", project_dir.display());
        let repository = Repository::init(&project_dir).with_context(|| "initialize git repository failed")?;
        let user = repository.current_user();
        let name = name.or_else(|| basename(&project_dir)).unwrap_or_default();
        let context = TemplateContext::new(user.name, user.email, name, edition, description);

        self.create_project_files(kind, &project_dir, &context)?;
        Ok(())
    }

    fn new_project(self, options: NewOptions) -> Result<()> {
        let NewOptions { path: project_dir, kind, name, edition, description } = options;

        if !project_dir.is_dir() {
            log::info!("creating directory `{}`", project_dir.display());
            fs::create_dir(&project_dir).with_context(|| format!("can't create directory `{:?}`", project_dir.display()))?;
        }
        Self::create_github_workflows_directory(&project_dir)?;
        Self::create_src_directory(&project_dir)?;

        log::info!("initializing git repository for `{}`", project_dir.display());
        let repository = Repository::init(&project_dir).with_context(|| "initialize git repository failed")?;
        let user = repository.current_user();
        let name = name.or_else(|| basename(&project_dir)).unwrap_or_default();
        let context = TemplateContext::new(user.name, user.email, name, edition, description);

        self.create_project_files(kind, &project_dir, &context)?;
        Ok(())
    }

    fn create_project_files(self, kind: NewProjectKind, project_dir: &Path, context: &TemplateContext) -> Result<()> {
        let Self { bin_file, lib_file, common_plain_files, templates } = self;
        for (dst, content) in common_plain_files {
            log::info!("writing file `{}`", dst.display());
            fs::write(project_dir.join(&dst), content).with_context(|| format!("write `{}` failed", dst.display()))?;
        }
        for (dst, content) in templates {
            log::info!("writing file `{}`", dst.display());
            fs::write(project_dir.join(&dst), render(content, context)).with_context(|| format!("write `{}` failed", dst.display()))?;
        }
        match kind {
            NewProjectKind::Bin => {
                log::info!("writing file `{}`", bin_file.0.display());
                fs::write(project_dir.join(&bin_file.0), bin_file.1).with_context(|| format!("write `{}` failed", bin_file.0.display()))?;
            }
            NewProjectKind::Lib => {
                log::info!("writing file `{}`", lib_file.0.display());
                fs::write(project_dir.join(&lib_file.0), lib_file.1).with_context(|| format!("write `{}` failed", lib_file.0.display()))?;
            }
        }

        Ok(())
    }

    fn create_github_workflows_directory(project_dir: &Path) -> Result<PathBuf> {
        let github_workflows_dir = project_dir.join(".github/workflows");
        if !github_workflows_dir.is_dir() {
            log::info!("creating directory `{}`", github_workflows_dir.display());
            fs::create_dir(&github_workflows_dir)
                .with_context(|| format!("can't create directory `{:?}`", github_workflows_dir.display()))?;
        }
        Ok(github_workflows_dir)
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

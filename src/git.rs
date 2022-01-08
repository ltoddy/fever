// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub trait RepositoryExt {
    // git config user.name
    fn get_username(&self) -> String;
    // git config user.email
    fn get_email(&self) -> String;
}

impl RepositoryExt for git2::Repository {
    fn get_username(&self) -> String {
        self.config()
            .and_then(|config| config.get_string("user.name"))
            .map_err(|_| log::warn!("can't get `user.name`, make sure to execute to `git config --global user.name <name>`"))
            .unwrap_or_default()
    }

    fn get_email(&self) -> String {
        self.config()
            .and_then(|config| config.get_string("user.email"))
            .map_err(|_| log::warn!("can't get `user.email`, make sure to execute to `git config --global user.email <email>`"))
            .unwrap_or_default()
    }
}

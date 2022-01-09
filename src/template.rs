// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)] // TODO: remove

use serde::Serialize;
use tinytemplate::TinyTemplate;

pub fn render<S>(content: &str, context: &S) -> String
where
    S: Serialize,
{
    let mut template = TinyTemplate::new();
    template.add_template("just", content).and_then(|_| template.render("just", context)).unwrap()
}

// TODO
struct Template<'template> {
    original_text: &'template str,
}

impl<'template> Template<'template> {}

#[cfg(test)]
pub mod test {}

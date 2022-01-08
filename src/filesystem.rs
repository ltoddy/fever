// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::path::Path;

// shell `basename` command
pub fn basename<P>(path: P) -> Option<String>
where
    P: AsRef<Path>,
{
    basename_impl(path.as_ref())
}

fn basename_impl(path: &Path) -> Option<String> {
    path.file_stem().and_then(|name| name.to_str()).map(|name| name.to_string())
}

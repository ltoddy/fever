// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::time::{SystemTime, UNIX_EPOCH};

const EPOCH_YEAR: i32 = 1970;
const A_DAY_SECS: i64 = 24 * 60 * 60;
const NORMAL_YEAR_SECS: i64 = 365 * A_DAY_SECS;
const LEAP_YEAR_SECS: i64 = 366 * A_DAY_SECS;

#[inline]
pub fn current_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn current_year() -> i32 {
    let mut now = current_timestamp() as i64;
    let mut year = EPOCH_YEAR;
    while now > 0 {
        match is_leap_year(year) {
            true if now >= LEAP_YEAR_SECS => {
                now -= LEAP_YEAR_SECS;
            }
            false if now >= NORMAL_YEAR_SECS => {
                now -= NORMAL_YEAR_SECS;
            }
            _ => break,
        }
        year += 1
    }
    year
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_current_year() {
        assert_eq!(2022, current_year())
    }
}

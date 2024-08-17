// SPDX-License-Identifier: GPL-3.0

use sandbox_macros::{some_name, sql};

fn main() {
    let name = some_name!("John");
    println!("{name:?}");

    let sql = sql!("SELECT * FROM post WHERE id=1");
    println!("{sql:?}")
}

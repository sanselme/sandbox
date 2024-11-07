// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/idioms/default.html

use std::{path::PathBuf, time::Duration};

// note: we can simply auto-derive Default here.
#[derive(Debug, Default, PartialEq)]
struct MyConfiguration {
    output: Option<PathBuf>,   // Option defaults to None
    search_path: Vec<PathBuf>, // Vecs default to empty vector
    timeout: Duration,         // Duration defaults to zero time
    check: bool,               // bool defaults to false
}

impl MyConfiguration {
    // todo: add setters here
}

fn main() {
    // construct a new instance with default values
    let mut conf = MyConfiguration::default();

    // do something with conf here
    conf.check = true;
    println!("conf = {conf:#?}");

    // partial initialization with default values, creates the same instance
    let conf1 = MyConfiguration {
        check: true,
        ..Default::default()
    };
    assert_eq!(conf, conf1);
}

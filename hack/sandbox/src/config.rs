// SPDX-License-Identifier: GPL-3.0

use std::path::PathBuf;
use std::time::Duration;

// note: we can simply auto-derive Default here.
#[derive(Default, Debug, PartialEq)]
pub struct Config {
    // Option defaults to None
    pub output: Option<PathBuf>,
    // Vecs default to empty vector
    pub search_path: Vec<PathBuf>,
    // Duration defaults to zero time
    pub timeout: Duration,
    // check defaults to false
    pub check: bool,
}

impl Config {
    // todo: add setters here
}

// pub struct Configs<T> {
//     pub data: Vec<Config>,
// }
//
// impl<T> Deref for Configs<T> {
//     type Target = [T];
//
//     fn deref(&self) -> &Self::Target {
//         todo!()
//     }
// }

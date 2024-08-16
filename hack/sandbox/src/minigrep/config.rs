// SPDX-License-Identifier: GPL-3.0

use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            None => return Err("Didn't get a query string"),
            Some(arg) => arg,
        };
        let file_path = match args.next() {
            None => return Err("Didn't get a file path"),
            Some(arg) => arg,
        };
        let ignore_case = match args.next() {
            None => false,
            Some(arg) => {
                "-i".eq(&arg) || "--insensitive=true".eq(&arg) || env::var("IGNORE_CASE").is_ok()
            }
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

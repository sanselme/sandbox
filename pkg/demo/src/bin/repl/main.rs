// SPDX-License-Identifier: GPL-3.0

mod cli;
mod util;

use crate::util::{readline, respond};
pub(crate) use cli::cli;
use std::io;
use std::io::Write;

fn main() -> Result<(), String> {
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(io::stdout(), "{err}").map_err(|e| e.to_string())?;
                io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

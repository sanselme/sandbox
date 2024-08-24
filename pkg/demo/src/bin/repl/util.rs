// SPDX-License-Identifier: GPL-3.0

use crate::cli;
use std::io;
use std::io::Write;

pub(crate) fn respond(line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let matches = cli()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;

    match matches.subcommand() {
        Some(("ping", _matches)) => {
            write!(io::stdout(), "Pong").map_err(|e| e.to_string())?;
            io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("quit", _matches)) => {
            write!(io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
            io::stdout().flush().map_err(|e| e.to_string())?;
            return Ok(true);
        }
        Some((name, _matches)) => unimplemented!("{name}"),
        None => unreachable!("Subcommand required"),
    }

    Ok(false)
}

pub(crate) fn readline() -> Result<String, String> {
    write!(io::stdout(), "$ ").map_err(|e| e.to_string())?;
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/idioms/on-stack-dyn-dispatch.html

use std::{fs::File, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = "/workspace/hack/boilerplate.txt".to_string();

    // We need to describe the type to get dynamic dispatch.
    let _readable: &mut dyn io::Read = if arg == "-" {
        &mut io::stdin()
    } else {
        &mut File::open(arg)?
    };

    // We still need to ascribe the type for dynamic dispatch.
    // let readable: Box<dyn io::Read> = if arg == "-" {
    //     Box::new(io::stdin())
    // } else {
    //     Box::new(File::open(arg)?)
    // };

    // todo: Read from `readable` here.

    Ok(())
}

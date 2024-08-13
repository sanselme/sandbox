// SPDX-License-Identifier: GPL-3.0

mod handler;

use std::io::prelude::*;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handler::handle_connection(stream);
    }
}

// SPDX-License-Identifier: GPL-3.0

mod handler;

use exercises::ThreadPool;
use std::{io::prelude::*, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handler::handle_connection(stream);
        });
    }
}

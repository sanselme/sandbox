// SPDX-License-Identifier: GPL-3.0

mod handler;
mod thread {
    pub mod pool;
    pub mod worker;
}

use crate::thread::pool;
use std::{io::prelude::*, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = pool::ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        // debug: remove .take(2)
        let stream = stream.unwrap();
        pool.execute(|| {
            handler::handle_connection(stream);
        });
    }

    println!("Shutting down...");
}

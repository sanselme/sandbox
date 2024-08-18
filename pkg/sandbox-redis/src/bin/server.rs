// SPDX-License-Identifier: GPL-3.0

use bytes::Bytes;
use mini_redis::{Command, Connection, Frame, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

async fn process(socket: TcpStream, db: Db) {
    // connection, provided by `sandbox-redis`, handles parsing frames
    // from the socket
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // write the response to the client
        connection.write_frame(&response).await.unwrap();
    }

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Got: {frame:?}");

        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Listening!");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // the second item contains the IP and Port of the new connection
        let (socket, _) = listener.accept().await?;

        // clone the handle to the hash map
        let db = db.clone();
        println!("Accepted!");

        // a new task is spawned for each inbound socket
        // the socket is moved to the new task and processed there
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

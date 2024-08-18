// SPDX-License-Identifier: GPL-3.0

use mini_redis::{Command, Connection, Frame, Result};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};

async fn process(socket: TcpStream) {
    // a hashmap is used to store data
    let mut db = HashMap::new();

    // the `Connection` lets us read/write redis **frames** instead of byte streams
    // the `Connection` type is defined by sandbox-redis
    let mut connection = Connection::new(socket);

    // use `read_frame` to receive a command from the connection
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                // the value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects dat to be of type `Bytes`
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

    loop {
        // the second item contains the IP and Port of the new connection
        let (socket, _) = listener.accept().await?;

        // a new task is spawned for each inbound socket
        // the socket is moved to the new task and processed there
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

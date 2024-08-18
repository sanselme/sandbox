// SPDX-License-Identifier: GPL-3.0

use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

/// Provided by the requester and used by the manager task to send
/// the command response back to the requester.
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

/// Multiple different commands are multiplexed over a single channel.
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        res: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        res: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        // open a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, res } => {
                    let r = client.get(&key).await;
                    let _ = res.send(r); // ignore errors
                }
                Command::Set { key, val, res } => {
                    let r = client.set(&key, val).await;
                    let _ = res.send(r); // ignore errors
                }
            }
        }
    });

    // spawn 2 tasks, 1 setting a value and
    // the other querying for key that was set
    let t1 = tokio::spawn(async move {
        let (res_tx, res_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foor".to_string(),
            res: res_tx,
        };

        // send the GET request
        tx.send(cmd).await.unwrap();

        // await the response
        let res = res_rx.await;
        println!("Got = {res:?}");
    });
    let t2 = tokio::spawn(async move {
        let (res_tx, res_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            res: res_tx,
        };

        // send the SET request
        tx2.send(cmd).await.unwrap();

        // await the response
        let res = res_rx.await;
        println!("Got = {res:?}");
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}

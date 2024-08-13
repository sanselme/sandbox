// SPDX-License-Identifier: GPL-3.0

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job! Executing...");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected! Shutting down...");
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

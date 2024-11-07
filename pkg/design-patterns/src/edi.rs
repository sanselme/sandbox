// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/idioms/rustdoc-init.html

use std::net::TcpStream;
use std::io::Error as SendErr;

struct Request;
struct Status;

struct Connection {
    name: String,
    stream: TcpStream,
}

impl Connection {
    /// Sends a request over the connection.
    ///
    /// # Example
    /// ```no_run
    /// # // Boilerplate are required to get an example working.
    /// # let stream = TcpStream::connect("127.0.0.1:8080");
    /// # let connection = Connection { name: "foo".to_owned(), stream };
    /// # let request = Request::new("RequestId", RequestType::Get, "payload");
    /// let response = connection.send_request(request);
    /// assert!(response.is_ok());
    /// ```
    fn send_request(&self, _request: Request) -> Result<Status, SendErr> {
        todo!()
    }

    /// Oh no, all that boilerplate needs to be repeated here!
    fn check_status(&self) -> Status {
        todo!()
    }

    /// Sends a request over the connection.
    ///
    /// # Example
    /// ```
    /// # fn call_send(connection: Connection, request: Request) {
    /// let response = connection.easier_send_request(request);
    /// assert!(response.is_ok());
    /// # }
    /// ```
    fn easier_send_request(&self, _request: Request) {
        todo!()
    }
}

// SPDX-License-Identifier: GPL-3.0

use api::v1alpha1::greeter_service_server::{GreeterService, GreeterServiceServer};
use api::v1alpha1::{SayHelloRequest, SayHelloResponse};
use std::error;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct DefaultGreeter {}

#[tonic::async_trait]
impl GreeterService for DefaultGreeter {
    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = SayHelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let addr = "[::1]:8080".parse()?;
    let greeter = DefaultGreeter::default();

    Server::builder()
        .add_service(GreeterServiceServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}

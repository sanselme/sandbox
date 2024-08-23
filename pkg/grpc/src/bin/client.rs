// SPDX-License-Identifier: GPL-3.0

use api::v1alpha1::greeter_service_client::GreeterServiceClient;
use api::v1alpha1::SayHelloRequest;
use std::error;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let mut client = GreeterServiceClient::connect("http://[::1]:8080").await?;
    let request = Request::new(SayHelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;
    println!("response={:?}", response);

    Ok(())
}

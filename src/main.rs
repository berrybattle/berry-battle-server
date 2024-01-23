use tonic::{transport::Server, Request, Response, Status};

use crate::hello::say_server::{Say, SayServer};
use hello::{SayRequest, SayResponse};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Default)]
pub struct MySay {}

#[tonic::async_trait]
impl Say for MySay {
    async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = SayResponse {
            message: format!("Hello {}!", request.get_ref().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    println!("Server listening on {}", addr);

    let say = MySay::default();
    Server::builder()
        .add_service(SayServer::new(say))
        .serve(addr)
        .await?;

    Ok(())
}

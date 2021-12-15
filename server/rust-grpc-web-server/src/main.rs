use simonsays::simon_says_server::{SimonSays, SimonSaysServer};
use simonsays::{Color, SimonRequest, SimonResponse};
use tonic::{transport::Server, Request, Response, Status};
pub mod simonsays {
    tonic::include_proto!("simonsays");
}
//use async_trait::async_trait;

#[derive(Debug, Default)]
pub struct MySimonSays {}

#[tonic::async_trait]
impl SimonSays for MySimonSays {
    async fn game(
        &self,
        request: Request<SimonRequest>,
    ) -> Result<Response<SimonResponse>, Status> {
        println!("get: {:?}", request);

        let replay = "OK".to_string();

        Ok(Response::new(replay))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50000".parse()?;
    let simonsays = MySimonSays::default();

    Server::builder()
        .add_service(SimonSays::new(simonsays))
        .serve(addr)
        .await?;
    Ok(())
}

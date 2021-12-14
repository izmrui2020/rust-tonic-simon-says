use anyhow::{Context, Result};
use tonic::{transport::Server, Request, Response, Status};

pub mod simonsays {
    tonic::include_proto!("simonsays");
}

#[derive(Debug, Default)]
pub struct SimonSays {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50000".parse()?;
    let simonsays = SimonSays::default();

    Server::builder()
        .add_service(SimonSays::new(simonsays))
        .serve(addr)
        .await?;
    Ok(())
}

use tonic::transport::Server;
use crate::services::echo_service;

mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse().unwrap();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(
            echo_service::messages::echo_service_server::EchoServiceServer::new(echo_service::EchoServiceImpl)
        )
        .serve(addr)
        .await?;

    Ok(())
}
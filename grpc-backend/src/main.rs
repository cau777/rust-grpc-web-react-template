use tonic::transport::Server;
use tower_http::trace::{TraceLayer};
use crate::logging::Logger;
use crate::services::echo_service;


mod services;
mod logging;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse().unwrap();
    println!("Server listening on {}", addr);


    Server::builder()
        .accept_http1(true)
        .layer(TraceLayer::new_for_grpc().on_request(Logger).on_response(Logger).on_failure(Logger))
        .add_service(
            // Use tonic_web::enable to deal with CORS and OPTIONS requests
            tonic_web::enable(echo_service::messages::echo_service_server::EchoServiceServer::new(echo_service::EchoServiceImpl))
        )
        .serve(addr)
        .await?;

    Ok(())
}
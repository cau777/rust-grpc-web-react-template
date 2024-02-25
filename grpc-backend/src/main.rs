use std::fmt::Debug;
use std::time::Duration;
use tonic::codegen::http::{Request, Response};
use tonic::transport::Server;
use tower_http::trace::{OnFailure, OnRequest, OnResponse, TraceLayer};
use tracing::Span;
use crate::services::echo_service;


mod services;

#[derive(Clone)]
struct Logger;

impl<B: Debug> OnResponse<B> for Logger {
    fn on_response(self, response: &Response<B>, latency: Duration, span: &Span) {
        println!("Response ({:?}):", response.status());
        println!("Response ({:?}): {:?}", response.status().canonical_reason(), response.status().as_str());
        println!("Headers: {:?}", response.headers());
        println!("Body: {:?}", response.body());
        println!();
    }
}

impl<B> OnFailure<B> for Logger {
    fn on_failure(&mut self, failure_classification: B, latency: Duration, span: &Span) {
        // TODO: better logging
        print!("Failure")
    }
}

impl<B> OnRequest<B> for Logger {
    fn on_request(&mut self, request: &Request<B>, span: &Span) {
        println!("Request");
        println!("Headers: {:?}", request.headers());
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse().unwrap();
    println!("Server listening on {}", addr);


    Server::builder()
        .accept_http1(true)
        .layer(TraceLayer::new_for_grpc().on_request(Logger).on_response(Logger).on_failure(Logger))
        .add_service(
            tonic_web::enable(echo_service::messages::echo_service_server::EchoServiceServer::new(echo_service::EchoServiceImpl))
        )
        .serve(addr)
        .await?;

    Ok(())
}
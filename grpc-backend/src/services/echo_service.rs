use tonic::{Request, Response, Status};
use crate::services::echo_service::messages::{EchoRequest, EchoResponse};

pub mod messages {
    tonic::include_proto!("echo");
}

#[derive(Default)]
pub struct EchoServiceImpl;

#[tonic::async_trait]
impl messages::echo_service_server::EchoService for EchoServiceImpl {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let reply = EchoResponse{
            message: request.into_inner().message,
        };
        Ok(Response::new(reply))
    }
}

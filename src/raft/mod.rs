use super::grpc_stub::raft::raft_server::Raft;
use super::grpc_stub::raft::{Message, Success};
use tonic::{Request, Response, Status};


pub struct RaftInstance{
}

#[tonic::async_trait]
impl Raft for RaftInstance{
    async fn step(&self, request: Request<Message>) -> Result<Response<Success>, Status> {
        log::info!("Got a request: {:?}", request);
        let reply = Success{};
        Ok(Response::new(reply))
    }
}

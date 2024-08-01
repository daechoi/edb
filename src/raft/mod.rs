use super::grpc_stub::raft::raft_server::Raft;
use super::grpc_stub::raft::{Message, Success};
use tonic::{Request, Response, Status};
use crossbeam_channel::Sender;

pub struct RaftInstance{
    local: Sender<Message>,
}

impl RaftInstance{
    pub fn new(local: Sender<Message>) -> Self{
        Self{local}
    }
}

#[tonic::async_trait]
impl Raft for RaftInstance{
    async fn step(&self, request: Request<Message>) -> Result<Response<Success>, Status> {
        log::info!("Got a request: {:?}", request);
        let reply = Success{};
        let message = request.into_inner();
        self.local.send(message).unwrap();
        Ok(Response::new(reply))
    }
}


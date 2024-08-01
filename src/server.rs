use tonic::{transport::Server, Request, Response, Status};
use crate::{configuration, grpc_stub};
use crate::raft::RaftInstance;
use std::collections::HashMap;
use std::net::SocketAddr;
use configuration::Settings;

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("edb_descriptor");

//pub type RpcResult<T> = std::result::Result<tonic::Response<T>, tonic::Status>;

/// Represents a connection
#[derive(Debug, Default)]
struct DbInstance {
    id: String,
}

#[tonic::async_trait]
impl grpc_stub::database_server::Database for DbInstance {
    async fn status(
        &self,
        request: Request<grpc_stub::StatusRequest>,
    ) -> Result<Response<grpc_stub::StatusResponse>, Status> {
        log::info!("Got a request: {:?}", request);
        let reply = grpc_stub::StatusResponse {
            id: self.id.clone(),
            version: env!("CARGO_PKG_VERSION").into(),
            time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        };

        Ok(Response::new(reply))
    }
}

/// DBServer Represents the server
#[derive(Debug)]
pub struct EDBServer {
    pub id: String,
    pub peers: HashMap<String, SocketAddr>,
    pub data_path: String,
    config: Settings,
}


impl EDBServer {
    pub fn new(config: Settings) -> Self {
        let peers = config.parse_peers().expect("Failed to parse peers");
        let id = config.server.id.clone();
        let data_path = config.server.data_path.clone();
        Self{ id, peers, config, data_path }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = "0.0.0.0";
        let addr = format!("{addr}:{}", self.config.server.port);
        let addr = addr.parse()?;

        let db = DbInstance { id: self.id.clone() };
        let rs = RaftInstance{};
        let reflection_server = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()?;

        Server::builder()
            .concurrency_limit_per_connection(self.config.server.threads)
            .add_service(grpc_stub::database_server::DatabaseServer::new(db))
            .add_service(grpc_stub::raft::raft_server::RaftServer::new(rs))
            .add_service(reflection_server)
            .serve(addr)
            .await?;
        Ok(())
    }
}

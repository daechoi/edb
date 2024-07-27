use tonic::{transport::Server, Request, Response, Status};
/*
pub mod grpc_edb {
    tonic::include_proto!("edb");
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("edb_descriptor");
}
*/

#[path = "service/edb.rs"]
pub mod grpc_edb;

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("edb_descriptor");
use grpc_edb::database_server::{Database, DatabaseServer};

//pub type RpcResult<T> = std::result::Result<tonic::Response<T>, tonic::Status>;

/// DBServer Represents the server
#[derive(Debug)]
pub struct EDBServer {
    pub id: String,
    pub addr: String,
    pub threads: usize,
}

impl Default for EDBServer {
    fn default() -> Self {
        Self { id: "default".into(), addr: "127.0.0.1:54301".into(), threads: 4 }
    }
}

impl EDBServer {
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = self.addr.parse().unwrap();
        let db = DbInstance { id: self.id.clone() };
        let reflection_server = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()?;

        Server::builder()
            .concurrency_limit_per_connection(self.threads)
            .add_service(DatabaseServer::new(db))
            .add_service(reflection_server)
            .serve(addr)
            .await?;
        Ok(())
    }
}

/// Represents a connection
#[derive(Debug, Default)]
struct DbInstance {
    id: String,
}

#[tonic::async_trait]
impl Database for DbInstance {
    async fn status(
        &self,
        request: Request<grpc_edb::StatusRequest>,
    ) -> Result<Response<grpc_edb::StatusResponse>, Status> {
        println!("Got a request: {:?}", request);
        let reply = grpc_edb::StatusResponse {
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

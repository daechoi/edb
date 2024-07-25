use tonic::{transport::Server, Request, Response, Status};
pub mod grpc_edb {
    tonic::include_proto!("edb");
}

use grpc_edb::database_server::{Database, DatabaseServer};

#[derive(Debug, Default)]
pub struct DBServer {
    pub id: String,
    pub addr: String,
    pub threads: usize,
}

impl DBServer {
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = self.addr.parse().unwrap();
        let db = DbInstance { id: self.id.clone() };
        Server::builder().add_service(DatabaseServer::new(db)).serve(addr).await?;
        Ok(())
    }
}
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

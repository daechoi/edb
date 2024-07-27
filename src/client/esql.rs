use env_logger::Env;
use edb::grpc_stub::{StatusRequest, database_client::DatabaseClient};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let channel = tonic::transport::Channel::from_static("http://127.0.0.1:54301")
        .connect()
        .await
        .unwrap();

    log::info!("Connected to server!");
    let client = DatabaseClient::new(channel);
    let mut handles = Vec::new();
    for i in 0..1000 {
        let mut client = client.clone();
        let j = tokio::spawn(async move {
            let request = tonic::Request::new(StatusRequest {});
            let response = client.status(request).await;
            log::info!("RESPONSE #{i}={:?}", response);
        });
        handles.push(j);

    }
    for handle in handles {
        handle.await?
    }

    Ok(())
}

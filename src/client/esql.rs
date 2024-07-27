mod grpc_client {
    tonic::include_proto!("edb");
}

use grpc_client::database_client::DatabaseClient;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://127.0.0.1:54301")
        .connect()
        .await
        .unwrap();

    println!("Connected to server!");
    let mut client = DatabaseClient::new(channel);
    
    let request = tonic::Request::new(grpc_client::StatusRequest {});

    let response = client.status(request).await?.into_inner();

    println!("RESPONSE={:?}", response);

    Ok(())
}

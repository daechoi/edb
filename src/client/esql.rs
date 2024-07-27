use env_logger::Env;
use std::str::FromStr;
use std::path::PathBuf;
use tonic::transport::Uri;
use edb::{configuration, grpc_stub::{database_client::DatabaseClient, StatusRequest}};
use clap::{arg,  value_parser, Command};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = configuration::get_configuration().expect("Failed to read configuration");
    let config = config.server;

    let addr = Uri::from_str(format!("http://{}",config.addr).as_str()).expect("Failed to parse the URL");
    let channel = tonic::transport::Channel::builder(addr)
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

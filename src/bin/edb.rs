use edb::server;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server = server::EDBServer::default();
    log::info!("Listening on {}", server.addr);
    server.start().await?;
    Ok(())
}

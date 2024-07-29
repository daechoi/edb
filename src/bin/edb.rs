use edb::{configuration::get_configuration, server};
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = get_configuration().expect("Failed to read configuration");
    env_logger::Builder::from_env(Env::default().default_filter_or(config.log_level)).init();

    let server = server::EDBServer::default();
    log::info!("Listening on {}", config.server.port);
    server.start().await?;
    Ok(())
}

use edb::{configuration::get_configuration, server};
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = get_configuration().expect("Failed to read configuration");
    let log_level = config.log_level.as_str();
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level)).init();

    log::info!("Listening on {}", config.server.port);

    let server = server::EDBServer::new(config);
    server.start().await?;
    Ok(())
}

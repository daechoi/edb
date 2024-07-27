use edb::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = server::EDBServer::default();
    println!("EDB listening on {}", server.addr);
    server.start().await?;
    Ok(())
}

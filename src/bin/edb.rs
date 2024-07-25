use edb::server;

extern crate nanoid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting EDB...");
    server::DBServer { id: nanoid::nanoid!(), addr: "127.0.0.1:5430".into(), threads: 4 }
        .start()
        .await?;
    Ok(())
}

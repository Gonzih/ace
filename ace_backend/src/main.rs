use ace_rpc::{ARPCClient, ARPCServer};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = ARPCClient::connect().await?;
    let server = ARPCServer::connect().await?;

    server.run().await?;

    let resp = client.test("BEBEBE".to_string()).await?;
    println!("resp {:?}", resp);

    Ok(())
}

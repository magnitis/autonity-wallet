mod autonity_client;
use crate::autonity_client::AutonityClient;
mod cli;
use crate::cli::Cli;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let autonity_client = AutonityClient::new("https://rpc2.piccadilly.autonity.org/");
    let cli = Cli::new(autonity_client);

    cli.run().await?;

    Ok(())
}

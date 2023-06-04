// This script just tests the Autonity Client
mod autonity_client;

use autonity_client::AutonityClient;

#[tokio::main]
async fn main() {
    let client = AutonityClient::new("http://localhost:8545");

    match client.get_network_version().await {
        Ok(version) => println!("Network version: {}", version),
        Err(e) => println!("Error getting network version: {}", e),
    }

    let account = "0x30aa985029fd8881B444128121Fd343f87755a35";
    match client.get_balance(account).await {
        Ok(balance) => println!("Balance for {}: {}", account, balance),
        Err(e) => println!("Error getting balance: {}", e),
    }

    let to = "0xe60666d768c7ef11397dcdfe20eb5d2c2fa6d244";
    let value = 1;
    let gas = 210000;

    match client.send_transaction(to, value, gas).await {
        Ok(tx_hash) => println!("Transaction hash: {:?}", tx_hash),
        Err(e) => println!("Error sending transaction: {}", e),
    }
}

use crate::AutonityClient;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::error::Error;

pub struct Cli {
    autonity_client: AutonityClient,
}

impl Cli {
    pub fn new(autonity_client: AutonityClient) -> Cli {
        Cli { autonity_client }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let commands = &[
            "Get network version",
            "Get balance",
            "Send transaction",
            "Exit",
        ];

        loop {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Please select a command")
                .default(0)
                .items(commands)
                .interact()
                .unwrap();

            match selection {
                0 => {
                    // Get network version
                    match self.autonity_client.get_network_version().await {
                        Ok(version) => println!("Network version: {}", version),
                        Err(e) => println!("Error getting network version: {}", e),
                    }
                }
                1 => {
                    // Get balance
                    let address: String = Input::new()
                        .with_prompt("Enter the address")
                        .interact()
                        .unwrap();

                    match self.autonity_client.get_balance(&address).await {
                        Ok(balance) => println!("ATN Balance for {}: {}", address, balance),
                        Err(e) => println!("Error getting balance: {}", e),
                    }
                }
                2 => {
                    // Send transaction
                    let to_address: String = Input::new()
                        .with_prompt("Enter the to_address")
                        .interact()
                        .unwrap();
                    let value: u64 = Input::new()
                        .with_prompt("Enter the value")
                        .interact()
                        .unwrap();
                    let gas: u64 = Input::new()
                        .with_prompt("Enter the gas")
                        .interact()
                        .unwrap();

                    match self
                        .autonity_client
                        .send_transaction(&to_address, value, gas)
                        .await
                    {
                        Ok(tx_hash) => println!("Transaction hash: {:?}", tx_hash),
                        Err(e) => println!("Error sending transaction: {}", e),
                    }
                }
                // TODO: Add more functionalities
                _ => break,
            }
        }

        Ok(())
    }
}

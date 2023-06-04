use dialoguer::Input;
use reqwest::Error;
use rustc_hex::FromHex;
use secp256k1::SecretKey;
use serde_json::json;
use std::fs::File;
use std::io::Read;
use web3::types::{Address, Bytes, TransactionParameters, H256, U256};

#[derive(Debug)]
pub struct AutonityClient {
    url: String,
}

impl AutonityClient {
    pub fn new(url: &str) -> AutonityClient {
        AutonityClient {
            url: url.to_string(),
        }
    }

    // Define your methods here. As an example, get the network version
    // Network version
    pub async fn get_network_version(&self) -> Result<String, Error> {
        let request_body = json!({
            "jsonrpc": "2.0",
            "method": "net_version",
            "params": [],
            "id": 1,
        });

        let client = reqwest::Client::new();
        let response = client.post(&self.url).json(&request_body).send().await?;

        let result: serde_json::Value = response.json().await?;
        Ok(result["result"].as_str().unwrap().to_string())
    }

    // Check balance
    pub async fn get_balance(&self, address: &str) -> Result<String, Error> {
        let request_body = json!({
            "jsonrpc": "2.0",
            "method": "eth_getBalance",
            "params": [address, "latest"],
            "id": 1,
        });

        let client = reqwest::Client::new();
        let response = client.post(&self.url).json(&request_body).send().await?;
        let result: serde_json::Value = response.json().await?;

        let balance_hex = result["result"].as_str().unwrap().to_string();
        // Remove "0x" prefix
        let balance_hex_trimmed = balance_hex.trim_start_matches("0x");
        let balance = i128::from_str_radix(&balance_hex_trimmed, 16)
            .unwrap()
            .to_string();

        Ok(balance)
    }

    pub async fn send_transaction(
        &self,
        to: &str,
        value: u64,
        gas: u64, // private_key_file: &str,
    ) -> Result<H256, Box<dyn std::error::Error>> {
        let transport = web3::transports::Http::new(&self.url)?;
        let web3 = web3::Web3::new(transport);

        // Parse the addres
        let to_address: Address = to.parse()?;

        // Get and read the private key file
        let private_key_file: String = Input::new()
            .with_prompt("Enter the keyfile absolute path")
            .interact()
            .expect("Failed to read input");

        let mut file = File::open(private_key_file)?;
        let mut key_content = String::new();
        file.read_to_string(&mut key_content)?;

        let private_key_bytes: Vec<u8> = key_content.trim_start_matches("0x").from_hex()?;
        let private_key = SecretKey::from_slice(&private_key_bytes)?;

        let _chain_id = 65100000;

        // Create the transaction
        let tx = TransactionParameters {
            to: Some(to_address),
            gas: U256::from(gas),
            gas_price: None,
            value: U256::from(value),
            data: Bytes::from("0x"),
            nonce: None,
            chain_id: Some(_chain_id),
            access_list: None,
            transaction_type: None,
        };

        // Sign the transaction
        let signed_tx = web3.accounts().sign_transaction(tx, &private_key).await?;

        // Send the transaction
        let tx_hash = web3
            .eth()
            .send_raw_transaction(signed_tx.raw_transaction)
            .await?;

        Ok(tx_hash)
    }
}

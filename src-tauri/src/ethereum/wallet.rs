use std::error::Error;
use std::io::BufWriter;
use std::str::FromStr;
use std::{fs::OpenOptions, io::BufReader};

use anyhow::Result;
use secp256k1::{PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use web3::{transports::WebSocket, types::Address, Web3};

use super::address;
#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub secret_key: String,
    pub public_key: String,
    pub public_address: String,
}

impl Wallet {
    pub fn new(secret_key: &SecretKey, pub_key: &PublicKey) -> Self {
        let addr: Address = address::generate_public_key_address(pub_key);
        Wallet {
            secret_key: secret_key.display_secret().to_string(),
            public_key: pub_key.to_string(),
            public_address: format!("{:?}", addr),
        }
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;
        let buf_writer = BufWriter::new(file);

        serde_json::to_writer_pretty(buf_writer, self)?;
        Ok(())
    }

    pub fn from_file(file_path: &str) -> Result<Wallet> {
        let file = OpenOptions::new().read(true).open(file_path)?;

        let buf_reader = BufReader::new(file);
        // Have to specify the type to be Wallet to tell serde_json what to deserialize to
        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }

    pub async fn get_balance(
        &self,
        web3_connection: &Web3<WebSocket>,
    ) -> Result<f64, Box<dyn Error>> {
        let wallet_address = Address::from_str(&self.public_address)?;
        let balance_wei = web3_connection.eth().balance(wallet_address, None).await?;
        let balance_eth = (balance_wei.as_u128() as f64) / 1_000_000_000_000_000_000.0;

        Ok(balance_eth)
    }
}

use crate::WALLET_FILE_PATH;

use std::error::Error;
use std::io::BufWriter;
use std::path::Path;
use std::str::FromStr;
use std::{
    fs::{File, OpenOptions},
    io::BufReader,
};

use anyhow::Result;
use secp256k1::{PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use web3::{transports::WebSocket, types::Address, Web3};

use super::hd_tree;
#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub account_name: String,
    pub secret_key: String,
    pub public_key: String,
    pub public_address: String,
}
#[allow(dead_code)]
impl Account {
    pub fn new(
        secret_key: &SecretKey,
        pub_key: &PublicKey,
        account_name: String,
        wallet_file: String,
    ) -> Result<(), String> {
        let file = File::open(WALLET_FILE_PATH);
        let addr: Address = hd_tree::public_key_to_address(pub_key);
        let new_account = Account {
            account_name,
            secret_key: secret_key.display_secret().to_string(),
            public_key: pub_key.to_string(),
            public_address: format!("{:?}", addr),
        };
        Ok(())
    }

    // pub fn get_account_from_file(account_name: &str) -> Result<Account> {
    //     let file = File::open(WALLET_FILE_PATH)?;
    //     Ok(())
    // }

    // pub fn save_to_file(&self, file_path: &str) -> Result<()> {
    //     let file = OpenOptions::new()
    //         .write(true)
    //         .create(true)
    //         .open(file_path)?;
    //     let buf_writer = BufWriter::new(file);

    //     serde_json::to_writer_pretty(buf_writer, self)?;
    //     Ok(())
    // }

    // pub fn from_file(file_path: &str) -> Result<Account> {
    //     let file = OpenOptions::new().read(true).open(file_path)?;

    //     let buf_reader = BufReader::new(file);
    //     // Have to specify the type to be an account to tell serde_json what to deserialize to
    //     let account: Account = serde_json::from_reader(buf_reader)?;
    //     Ok(account)
    // }

    pub async fn get_eth_balance(
        &self,
        web3_connection: &Web3<WebSocket>,
    ) -> Result<f64, Box<dyn Error>> {
        let wallet_address = Address::from_str(&self.public_address)?;
        let balance_wei = web3_connection.eth().balance(wallet_address, None).await?;
        let balance_eth = (balance_wei.as_u128() as f64) / 1_000_000_000_000_000_000.0;

        Ok(balance_eth)
    }
}

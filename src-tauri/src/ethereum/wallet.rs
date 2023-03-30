use secp256k1::{PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::BufWriter;
use std::str::FromStr;
use std::{fs::OpenOptions, io::BufReader};
use web3::types::Address;

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

    pub fn save_to_file(&self, file_path: &str) -> Result<(), std::io::Error> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;
        let buf_writer = BufWriter::new(file);

        match serde_json::to_writer_pretty(buf_writer, self) {
            Ok(()) => Ok(()),
            Err(error) => panic!("Error serializing to json: {:?}", error),
        }
    }

    pub fn from_file(file_path: &str) -> Result<Wallet, Box<dyn Error>> {
        let file = OpenOptions::new().read(true).open(file_path)?;

        let buf_reader = BufReader::new(file);
        // Have to specify the type to be Wallet to tell serde_json what to deserialize to
        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }
}

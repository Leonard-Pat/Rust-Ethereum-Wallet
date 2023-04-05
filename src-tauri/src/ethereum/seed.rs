use super::hd_tree::{self, AllowedWordCount};
use anyhow::Result;
use bip39::{Language, Mnemonic};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct Seed {
    #[serde(serialize_with = "<[_]>::serialize")]
    pub seed: Vec<u8>,
}

impl Seed {
    pub fn new(word_count: AllowedWordCount, passphrase: Option<String>) -> Seed {
        let mnemonic = hd_tree::generate_mnemonic(word_count);
        let seed = hd_tree::generate_seed(&mnemonic, passphrase);
        Seed {
            seed: seed.to_vec(),
        }
    }

    pub fn restore_from_phrase(words: &str, passphrase: Option<String>) -> Result<Seed> {
        let mnemonic = Mnemonic::parse_in_normalized(Language::English, words)?;
        let seed = hd_tree::generate_seed(&mnemonic, passphrase);
        let seed = Seed {
            seed: seed.to_vec(),
        };
        Ok(seed)
    }
}

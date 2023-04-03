use std::ops::Deref;

use anyhow::Result;
use bip32::secp256k1::ecdsa::SigningKey;
use bip32::{ChildNumber, Prefix, XPrv};
use bip39::{Language, Mnemonic};
use secp256k1::rand;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use tiny_keccak::{Hasher, Keccak};
use web3::types::Address;

pub fn create_mnemonic() -> Result<()> {
    // Generate 24 word mnemonic phrase
    let mnemonic_phrase = Mnemonic::generate_in(Language::English, 12).unwrap();

    // Generate seed from mnemonic phrase
    let seed = mnemonic_phrase.to_seed("");

    // Derive the root extended private key from the seed
    let root_xprv = XPrv::new(&seed).unwrap();

    // Derive a child `XPrv` using the provided BIP32 derivation path
    let child_path = "m/44'/60'/0'/0";
    let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse()?)?
        .derive_child(ChildNumber::new(0, false)?)?;

    // Get the `XPub` associated with `child_xprv`.
    // let child_xpub = child_xprv.public_key();

    // Get the ECDSA/secp256k1 signing and verification keys for the xprv and xpub
    let signing_key = child_xprv.private_key();

    let mut eth_priv_key = [0u8; 32];
    eth_priv_key.copy_from_slice(&signing_key.to_bytes()[..]);
    let secret_key = SecretKey::from_slice(&eth_priv_key)?;

    // Create a Secp256k1 context
    let secp256k1 = Secp256k1::new();

    // Derive the public key from the signing key
    let public_key = PublicKey::from_secret_key(&secp256k1, &secret_key);

    let public_key_serialized =
        PublicKey::from_secret_key(&secp256k1, &secret_key).serialize_uncompressed();

    // create keccak hasher
    let mut keccak_hasher = Keccak::v256();
    let mut output = [0u8; 32];
    keccak_hasher.update(&public_key_serialized[1..]);
    keccak_hasher.finalize(&mut output);

    // use last 20bytes to generate address
    let ethereum_address = Address::from_slice(&output[12..]);

    println!("Mnemonic: {}", mnemonic_phrase);
    println!("Seed: {}", hex::encode(seed));
    let masterkey: String = root_xprv.to_string(Prefix::XPRV).deref().to_string();
    println!("Master Private Key: {:?}", masterkey);

    // Print out the Ethereum private key and address
    println!(
        "Ethereum Private Key: 0x{}",
        secret_key.display_secret().to_string()
    );
    println!("Ethereum Public Key: 0x{}", public_key.to_string());
    println!("Ethereum Address: {:?}", ethereum_address);

    let another = secret_key_to_address(&signing_key);
    println!("this: {:?}", another);

    Ok(())
}

/// Converts a K256 SigningKey to an Ethereum Address
pub fn secret_key_to_address(secret_key: &SigningKey) -> Address {
    let public_key = secret_key.verifying_key();
    let public_key = public_key.to_encoded_point(/* compress = */ false);
    let public_key = public_key.as_bytes();
    debug_assert_eq!(public_key[0], 0x04);
    // create keccak hasher
    let mut keccak_hasher = Keccak::v256();
    let mut output = [0u8; 32];
    keccak_hasher.update(&public_key[1..]);
    keccak_hasher.finalize(&mut output);

    // use last 20bytes to generate address
    Address::from_slice(&output[12..])
}

pub fn create_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let (sk1, pk1) = secp.generate_keypair(&mut rand::thread_rng());
    (sk1, pk1)
}

pub fn generate_public_key_address(pub_key: &PublicKey) -> Address {
    // generate pub key and serialize it

    let public_key = pub_key.serialize_uncompressed();

    // check first byte is hex 4 / 0x04
    debug_assert_eq!(public_key[0], 0x04);

    // create keccak hasher
    let mut keccak_hasher = Keccak::v256();
    let mut output = [0u8; 32];
    keccak_hasher.update(&public_key[1..]);
    keccak_hasher.finalize(&mut output);

    // use last 20bytes to generate address
    Address::from_slice(&output[12..])
}

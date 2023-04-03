use anyhow::Result;
use bip32::secp256k1::ecdsa::{SigningKey, VerifyingKey};
use bip32::{ChildNumber, ExtendedPrivateKey, ExtendedPublicKey, XPrv};
use bip39::{Language, Mnemonic};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use tiny_keccak::{Hasher, Keccak};
use web3::types::Address;

pub fn generate_mnemonic(word_count: usize) -> Mnemonic {
    // Generate a word mnemonic phrase
    Mnemonic::generate_in(Language::English, word_count).unwrap()
}

pub fn generate_seed(mnemonic_phrase: &Mnemonic, passphrase: Option<String>) -> [u8; 64] {
    // Generate a seed from mnemonic phrase where the passphrase is optional
    // From docs as_deref:
    // Converts from Option<T> (or &Option<T>) to Option<&T::Target>.
    // Leaves the original Option in-place, creating a new one with a reference to the original one, additionally
    // needed for unwrap_or to return a string slice from an Option<String>
    mnemonic_phrase.to_seed(passphrase.as_deref().unwrap_or(""))
}

// still unsure on this
// pub fn generate_master_private_key(seed: &[u8; 64]) -> ExtendedPrivateKey<SigningKey> {
//     // Derive the root extended private key from a seed
//     XPrv::new(seed).unwrap()
// }

pub fn derive_child_extended_keys(
    seed: [u8; 64],
    child_path: &str,
    child_index: u32,
) -> Result<(
    ExtendedPrivateKey<SigningKey>,
    ExtendedPublicKey<VerifyingKey>,
)> {
    // Derive a child `XPrv` using the provided BIP32 derivation path
    // hardened is hardcoded to false
    let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse()?)?
        .derive_child(ChildNumber::new(child_index, false)?)?;

    // Get the `XPub` associated with `child_xprv`.
    let child_xpub = child_xprv.public_key();
    Ok((child_xprv, child_xpub))
}

// get Secret and Public keys from SigningKey
// example of getting signing key from child xprv:
// let signing_key = child_xprv.private_key();
pub fn derive_child_keys(signing_key: &SigningKey) -> Result<(SecretKey, PublicKey)> {
    let mut eth_priv_key = [0u8; 32];
    eth_priv_key.copy_from_slice(&signing_key.to_bytes()[..]);
    let secret_key = SecretKey::from_slice(&eth_priv_key)?;

    // Create a Secp256k1 context
    let secp256k1 = Secp256k1::new();

    // Derive the public key from the signing key
    let public_key = PublicKey::from_secret_key(&secp256k1, &secret_key);

    Ok((secret_key, public_key))
}

/// Converts a K256 SigningKey to an Ethereum Address
pub fn secret_key_to_address(pub_key: &PublicKey) -> Address {
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

pub fn full_flow() -> Result<()> {
    // // Derive a child `XPrv` using the provided BIP32 derivation path
    let child_path = "m/44'/60'/0'/0";
    let mnemonic_phrase = generate_mnemonic(12);
    let seed = generate_seed(&mnemonic_phrase, Option::None);
    let (xprv, _) = derive_child_extended_keys(seed, child_path, 0)?;
    let (secret_key, public_key) = derive_child_keys(xprv.private_key())?;
    let ethereum_address = secret_key_to_address(&public_key);

    println!("Mnemonic: {}", mnemonic_phrase);
    println!("Seed: {}", hex::encode(seed));
    println!(
        "Ethereum Private Key: 0x{}",
        secret_key.display_secret().to_string()
    );
    println!("Ethereum Public Key: 0x{}", public_key.to_string());
    println!("Ethereum Address: {:?}", ethereum_address);
    Ok(())
}

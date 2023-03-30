use secp256k1::rand;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use tiny_keccak::{Hasher, Keccak};
use web3::types::Address;

pub fn create_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let (sk1, pk1) = secp.generate_keypair(&mut rand::thread_rng());
    (sk1, pk1)
}

pub fn generate_public_key_address(pub_key: &PublicKey) -> Address {
    // generate pub key and serialize it

    let public_key = pub_key.serialize_uncompressed();

    // create keccak hasher
    let mut keccak_hasher = Keccak::v256();
    let mut output = [0u8; 32];
    keccak_hasher.update(&public_key[1..]);
    keccak_hasher.finalize(&mut output);

    // check first byte is hex 4 / 0x04
    debug_assert_eq!(public_key[0], 0x04);

    // use last 20bytes to generate address
    Address::from_slice(&output[12..])
}

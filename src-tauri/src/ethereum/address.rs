use secp256k1::rand;
use secp256k1::Secp256k1;
use tiny_keccak::{Hasher, Keccak};
use web3::types::Address;

#[tauri::command]
pub fn public_key_address() -> String {
    let secp = Secp256k1::new();
    let (sk1, pk1) = secp.generate_keypair(&mut rand::thread_rng());
    (sk1, pk1);
    let public_key = pk1.serialize_uncompressed();
    let mut keccak_hasher = Keccak::v256();
    let mut output = [0u8; 32];
    keccak_hasher.update(&public_key[1..]);
    keccak_hasher.finalize(&mut output);
    // check first byte is hex 4 / 0x04
    debug_assert_eq!(public_key[0], 0x04);

    // let hash = keccakf(&public_key[1..]);
    Address::from_slice(&output[12..]).to_string()
}

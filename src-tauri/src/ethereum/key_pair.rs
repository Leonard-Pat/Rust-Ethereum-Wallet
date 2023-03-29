use secp256k1::rand;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let (sk1, pk1) = secp.generate_keypair(&mut rand::thread_rng());
    (sk1, pk1)
}

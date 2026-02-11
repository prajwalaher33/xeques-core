use sha2::{Sha256, Digest};
use pqcrypto_dilithium::dilithium5;
use pqcrypto_traits::sign::PublicKey;

pub struct Identity {
    pub public_key: dilithium5::PublicKey,
    pub secret_key: dilithium5::SecretKey,
    pub id: String,
    pub device_id: String,
}

fn hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

pub fn load_or_create() -> Identity {
    let (pk, sk) = dilithium5::keypair();
    let id = hash(pk.as_bytes());
    let device_id = id.clone();

    Identity {
        public_key: pk,
        secret_key: sk,
        id,
        device_id,
    }
}

use pqcrypto_dilithium::dilithium5::*;
use pqcrypto_traits::sign::*;
use std::fs;

pub fn load_or_create_node_key(node_id: &str) -> (PublicKey, SecretKey) {

    let path = format!("keys/node_{}.key", node_id);

    if let Ok(data) = fs::read(&path) {
        let sk = SecretKey::from_bytes(&data).unwrap();
        let pk = sk.public_key();
        return (pk, sk);
    }

    let (pk, sk) = keypair();

    fs::create_dir_all("keys").ok();
    fs::write(&path, sk.as_bytes()).unwrap();

    (pk, sk)
}

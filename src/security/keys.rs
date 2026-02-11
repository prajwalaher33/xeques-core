use std::fs;
use std::path::Path;
use pqcrypto_dilithium::dilithium5::{keypair, PublicKey, SecretKey};
use pqcrypto_traits::sign::{PublicKey as PQPublicTrait, SecretKey as PQSecretTrait};

pub struct Keypair {
    pub public: PublicKey,
    pub secret: SecretKey,
}

pub fn load_or_create_keypair(device_id: &str) -> Keypair {
    let keydir = Path::new("keys");
    if !keydir.exists() {
        fs::create_dir_all(keydir).unwrap();
    }

    let pub_path = keydir.join(format!("{}.pub", device_id));
    let sec_path = keydir.join(format!("{}.sec", device_id));

    if pub_path.exists() && sec_path.exists() {
        let pub_bytes = fs::read(pub_path).unwrap();
        let sec_bytes = fs::read(sec_path).unwrap();

        // use the trait's associated constructor
        let public = PQPublicTrait::from_bytes(&pub_bytes).expect("invalid public bytes");
        let secret = PQSecretTrait::from_bytes(&sec_bytes).expect("invalid secret bytes");

        return Keypair { public, secret };
    }

    let (public, secret) = keypair();

    // write raw bytes to files (instance method as_bytes() is available because the trait is in scope)
    fs::write(&pub_path, public.as_bytes()).unwrap();
    fs::write(&sec_path, secret.as_bytes()).unwrap();

    Keypair { public, secret }
}

use std::fs;
use std::collections::BTreeMap;

use pqcrypto_dilithium::dilithium5::{
    detached_sign,
    verify_detached_signature,
    DetachedSignature,
    PublicKey,
    SecretKey,
};

use pqcrypto_traits::sign::{
    PublicKey as PQPublicTrait,
    DetachedSignature as PQDetachedTrait,
};

/// Sign message using Dilithium5 secret key
pub fn sign(secret: &SecretKey, message: &[u8]) -> Vec<u8> {
    let sig: DetachedSignature = detached_sign(message, secret);
    sig.as_bytes().to_vec()
}

/// Verify signature using provided public key
pub fn verify_signature(
    public: &PublicKey,
    message: &[u8],
    signature: &[u8],
) -> bool {

    match PQDetachedTrait::from_bytes(signature) {
        Ok(sig) => verify_detached_signature(&sig, message, public).is_ok(),
        Err(_) => false,
    }
}

/// Verify signature by loading public key from registry
pub fn verify_signature_from_registry(
    device_id: &str,
    message: &[u8],
    signature: &[u8],
) -> bool {

    let reg_path = "keys/registry.json";
    if !std::path::Path::new(reg_path).exists() {
        return false;
    }

    let reg_txt = fs::read_to_string(reg_path).unwrap();
    let map: BTreeMap<String,String> =
        serde_json::from_str(&reg_txt).unwrap_or_default();

    if !map.contains_key(device_id) {
        return false;
    }

    let pub_path = format!("keys/{}.pub", device_id);
    if !std::path::Path::new(&pub_path).exists() {
        return false;
    }

    let pub_bytes = fs::read(pub_path).unwrap();

    let public: PublicKey =
        PQPublicTrait::from_bytes(&pub_bytes)
            .expect("invalid public key");

    verify_signature(&public, message, signature)
}

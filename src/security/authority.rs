use std::fs;
use std::collections::BTreeMap;

use pqcrypto_dilithium::dilithium5::PublicKey;
use pqcrypto_traits::sign::PublicKey as PQPublicTrait;

#[derive(Clone)]
pub struct Authority {
    pub role: String,
    pub public_key: PublicKey,
}

pub fn load_authority(authority_id: &str) -> Option<Authority> {

    let path = "keys/authorities.json";
    if !std::path::Path::new(path).exists() {
        return None;
    }

    let txt = fs::read_to_string(path).ok()?;
    let map: BTreeMap<String, String> =
        serde_json::from_str(&txt).ok()?;

    if !map.contains_key(authority_id) {
        return None;
    }

    let pub_path = format!("keys/{}.pub", authority_id);
    let pub_bytes = fs::read(pub_path).ok()?;

    let public: PublicKey =
        PQPublicTrait::from_bytes(&pub_bytes).ok()?;

    Some(Authority {
        role: map.get(authority_id)?.clone(),
        public_key: public,
    })
}

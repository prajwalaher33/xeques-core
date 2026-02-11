use std::fs;
use std::collections::BTreeMap;
use pqcrypto_traits::sign::PublicKey as PQPublicTrait;
use pqcrypto_dilithium::dilithium5::PublicKey;

pub fn register_device(device_id: &str, public: &PublicKey) {
    let reg_path = "keys/registry.json";

    let mut map: BTreeMap<String,String> = if std::path::Path::new(reg_path).exists() {
        let txt = fs::read_to_string(reg_path).unwrap();
        serde_json::from_str(&txt).unwrap_or_default()
    } else {
        BTreeMap::new()
    };

    let hex_key = hex::encode(public.as_bytes());
    map.insert(device_id.to_string(), hex_key);

    let txt = serde_json::to_string_pretty(&map).unwrap();
    fs::write(reg_path, txt).unwrap();
}

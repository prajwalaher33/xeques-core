use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const FINALITY_FILE: &str = "keys/finality.json";

pub fn load_finality() -> BTreeMap<String, u64> {
    if !Path::new(FINALITY_FILE).exists() {
        return BTreeMap::new();
    }

    let txt = fs::read_to_string(FINALITY_FILE).unwrap_or_default();
    serde_json::from_str(&txt).unwrap_or_default()
}

pub fn save_finality(map: &BTreeMap<String, u64>) {
    fs::create_dir_all("keys").ok();
    let txt = serde_json::to_string_pretty(map).unwrap();
    fs::write(FINALITY_FILE, txt).unwrap();
}

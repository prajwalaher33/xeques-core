use std::fs;
use crate::block::Block;

pub fn load() -> Vec<Block> {
    if let Ok(data) = fs::read_to_string("chain.json") {
        serde_json::from_str(&data).unwrap_or(vec![])
    } else {
        vec![]
    }
}

pub fn save(chain: &Vec<Block>) {
    let data = serde_json::to_string_pretty(chain).unwrap();
    fs::write("chain.json", data).unwrap();
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub command_hash: String,
    pub prev_hash: String,
    pub algorithm: String,
}

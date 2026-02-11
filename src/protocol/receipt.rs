use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ArbitrationReceipt {
    pub device_id: String,
    pub sequence: u64,
    pub authority_id: String,
    pub role: String,
    pub timestamp: u64,
    pub outcome: String,
    pub signature: String,
}

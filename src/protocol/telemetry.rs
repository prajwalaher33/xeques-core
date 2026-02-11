use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Telemetry {
    pub device_id: String,
    pub current_mode: String,
    pub last_sequence: u64,
}

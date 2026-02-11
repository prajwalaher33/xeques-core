use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SatelliteCommandType {
    SetMode { mode: String },
    ExecuteBurn { delta_v: f64 },
    DeployPayload,
    Reset,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Command {
    pub authority_id: String,
    pub device_id: String,
    pub sequence: u64,
    pub timestamp: u64,
    pub expires_at: u64,
    pub action: SatelliteCommandType,
}

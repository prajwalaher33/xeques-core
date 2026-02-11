use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FederationDecision {
    pub device_id: String,
    pub sequence: u64,
    pub decision_hash: String,
    pub node_id: String,
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FederationCertificate {
    pub device_id: String,
    pub sequence: u64,
    pub decision_hash: String,
    pub participating_nodes: Vec<String>,
}

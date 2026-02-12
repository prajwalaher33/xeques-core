use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConsensusMessage {
    Propose {
        device_id: String,
        sequence: u64,
        hash: String,
        proposer: String,
    },
    Prevote {
        device_id: String,
        sequence: u64,
        hash: String,
        voter: String,
    },
    Precommit {
        device_id: String,
        sequence: u64,
        hash: String,
        voter: String,
    }
}

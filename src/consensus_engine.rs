use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::protocol::consensus::ConsensusMessage;

pub struct ConsensusEngine {
    votes: Arc<Mutex<HashMap<(String, u64, String), Vec<String>>>>,
    federation_size: usize,
}

impl ConsensusEngine {

    pub fn new(size: usize) -> Self {
        Self {
            votes: Arc::new(Mutex::new(HashMap::new())),
            federation_size: size,
        }
    }

    pub fn handle_message(&self, msg: ConsensusMessage) -> bool {

        let (device, seq, hash, voter) = match msg {
            ConsensusMessage::Propose { device_id, sequence, hash, proposer } =>
                (device_id, sequence, hash, proposer),

            ConsensusMessage::Prevote { device_id, sequence, hash, voter } =>
                (device_id, sequence, hash, voter),

            ConsensusMessage::Precommit { device_id, sequence, hash, voter } =>
                (device_id, sequence, hash, voter),
        };

        let mut locked = self.votes.lock().unwrap();

        let key = (device.clone(), seq, hash.clone());

        let entry = locked.entry(key).or_insert(Vec::new());

        if !entry.contains(&voter) {
            entry.push(voter);
        }

        let quorum = (self.federation_size * 2 / 3) + 1;

        if entry.len() >= quorum {
            println!("🟢 2/3 QUORUM REACHED for {} seq={}", device, seq);
            return true;
        }

        false
    }
}

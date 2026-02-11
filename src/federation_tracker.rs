use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::protocol::federation::{FederationDecision, FederationCertificate};
use crate::security::vote_crypto;
use pqcrypto_dilithium::dilithium5::PublicKey;

pub struct MajorityTracker {
    pub votes: Arc<Mutex<HashMap<(String, u64), Vec<FederationDecision>>>>,
    pub federation_size: usize,
    pub validator_keys: HashMap<String, PublicKey>,
}

impl MajorityTracker {

    pub fn new(size: usize) -> Self {
        Self {
            votes: Arc::new(Mutex::new(HashMap::new())),
            federation_size: size,
            validator_keys: HashMap::new(),
        }
    }

    pub fn register_validator(&mut self, node_id: String, pk: PublicKey) {
        self.validator_keys.insert(node_id, pk);
    }

    pub fn add_vote(&self, decision: FederationDecision)
        -> Option<FederationCertificate>
    {

        let pk = match self.validator_keys.get(&decision.node_id) {
            Some(p) => p,
            None => {
                println!("⚠ Unknown validator {}", decision.node_id);
                return None;
            }
        };

        let payload = format!(
            "{}:{}:{}",
            decision.device_id,
            decision.sequence,
            decision.decision_hash
        );

        if !vote_crypto::verify_vote(pk, payload.as_bytes(), &decision.signature) {
            println!("❌ Invalid vote signature from {}", decision.node_id);
            return None;
        }

        println!("✅ Verified vote from {}", decision.node_id);

        let mut locked = self.votes.lock().unwrap();
        let key = (decision.device_id.clone(), decision.sequence);

        let entry = locked.entry(key.clone()).or_insert(Vec::new());
        entry.push(decision.clone());

        let majority = (self.federation_size / 2) + 1;

        let mut hash_count: HashMap<String, Vec<String>> = HashMap::new();

        for vote in entry.iter() {
            hash_count
                .entry(vote.decision_hash.clone())
                .or_insert(Vec::new())
                .push(vote.node_id.clone());
        }

        for (hash, nodes) in hash_count {
            if nodes.len() >= majority {
                println!("🌍 Federation majority reached for {} seq={}", key.0, key.1);

                return Some(FederationCertificate {
                    device_id: key.0,
                    sequence: key.1,
                    decision_hash: hash,
                    participating_nodes: nodes,
                });
            }
        }

        None
    }
}

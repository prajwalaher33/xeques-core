use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedCommand {
    pub command: Vec<u8>,
    pub signature: String,
}

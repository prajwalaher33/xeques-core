use clap::Parser;
use std::sync::Arc;

mod protocol;
mod security;
mod node_config;
mod consensus_engine;
mod federation_network;

use node_config::NodeConfig;
use consensus_engine::ConsensusEngine;

fn main() {

    let config = NodeConfig::parse();

    println!("?? Node {} starting...", config.node_id);

    let peers: Vec<String> =
        config.peers.split(",").map(|s| s.to_string()).collect();

    let engine = Arc::new(ConsensusEngine::new(config.federation_size));

    federation_network::start_federation(
        &config.bind,
        peers.clone(),
        engine.clone(),
    );

    println!("? BFT Federation Ready.");
}

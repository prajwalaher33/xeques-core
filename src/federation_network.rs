use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::sync::Arc;

use crate::protocol::consensus::ConsensusMessage;
use crate::consensus_engine::ConsensusEngine;

pub fn start_federation(
    bind_addr: &str,
    peers: Vec<String>,
    engine: Arc<ConsensusEngine>,
) {

    let listener = TcpListener::bind(bind_addr).expect("Bind failed");

    println!("🌍 Federation listening on {}", bind_addr);

    let engine_clone = engine.clone();

    thread::spawn(move || {
        for stream in listener.incoming() {
            if let Ok(mut socket) = stream {

                let mut buffer = Vec::new();
                socket.read_to_end(&mut buffer).unwrap();

                if let Ok(msg) = serde_json::from_slice::<ConsensusMessage>(&buffer) {
                    engine_clone.handle_message(msg);
                }
            }
        }
    });

    for peer in peers {
        println!("🔗 Connected to peer {}", peer);
    }
}

pub fn broadcast(
    peers: &[String],
    message: &ConsensusMessage
) {

    let data = serde_json::to_vec(message).unwrap();

    for peer in peers {
        if let Ok(mut stream) = TcpStream::connect(peer) {
            let _ = stream.write_all(&data);
        }
    }
}

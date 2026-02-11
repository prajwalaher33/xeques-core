use std::net::TcpStream;
use std::io::Write;

use crate::protocol::federation::FederationDecision;

pub fn broadcast_to_peers(
    peers: &Vec<String>,
    decision: &FederationDecision,
) {

    let data = serde_json::to_vec(decision).unwrap();

    for peer in peers {
        if let Ok(mut stream) = TcpStream::connect(peer) {
            let _ = stream.write_all(&data);
        }
    }
}

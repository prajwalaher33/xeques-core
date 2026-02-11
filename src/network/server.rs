use std::net::TcpListener;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use crate::protocol::command::Command;
use crate::protocol::queue::QueuedCommand;
use crate::protocol::signed::SignedCommand;
use crate::protocol::state::DeviceState;
use crate::protocol::logger::log_rejection;
use crate::security::{verify, authority};

pub fn run_server(
    _state: Arc<Mutex<DeviceState>>,
    queue: Arc<Mutex<Vec<QueuedCommand>>>,
) {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("bind failed");

    println!("🚦 Satellite Node listening (Queued Mode)");

    for stream in listener.incoming() {
        if let Ok(mut s) = stream {

            let mut buf = Vec::new();
            if s.read_to_end(&mut buf).is_err() { continue; }
            if buf.is_empty() { continue; }

            let txt = String::from_utf8_lossy(&buf).to_string();

            if let Ok(scmd) =
                serde_json::from_str::<SignedCommand>(&txt)
            {
                if let Ok(cmd) =
                    serde_json::from_slice::<Command>(&scmd.command)
                {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    if now > cmd.expires_at {
                        log_rejection("Expired");
                        continue;
                    }

                    let auth =
                        match authority::load_authority(&cmd.authority_id) {
                            Some(a) => a,
                            None => {
                                log_rejection("Unknown authority");
                                continue;
                            }
                        };

                    let sig_bytes =
                        STANDARD.decode(&scmd.signature).unwrap();

                    if !verify::verify_signature(
                        &auth.public_key,
                        &scmd.command,
                        &sig_bytes,
                    ) {
                        log_rejection("Signature failed");
                        continue;
                    }

                    let allowed = match auth.role.as_str() {
                        "Operator" => true,
                        "Limited" => {
                            matches!(
                                cmd.action,
                                crate::protocol::command::
                                    SatelliteCommandType::Reset
                            )
                        }
                        "Emergency" => true,
                        _ => false,
                    };

                    if !allowed {
                        log_rejection("Policy violation");
                        continue;
                    }

                    let mut locked = queue.lock().unwrap();
                    locked.push(QueuedCommand {
                        command: cmd.clone(),
                        received_at: now,
                        role: auth.role.clone(),
                    });

                    println!(
                        "[QUEUED] authority={} device={} seq={}",
                        cmd.authority_id,
                        cmd.device_id,
                        cmd.sequence
                    );
                }
            }
        }
    }
}

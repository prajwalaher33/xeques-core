use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::BTreeMap;

use crate::protocol::command::{Command, SatelliteCommandType};
use crate::protocol::signed::SignedCommand;
use crate::security::keys;
use crate::network::client;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub fn run_simulation(_n_devices: usize, commands_per_device: usize, delay_ms: u64) {

    let addr = "127.0.0.1:7878";

    // -------- Create Authorities --------
    let authorities = vec![
        ("ground-alpha", "Operator"),
        ("ground-beta", "Limited"),
        ("ground-gamma", "Emergency"),
    ];

    std::fs::create_dir_all("keys").ok();

    let mut auth_map = BTreeMap::new();

    for (id, role) in &authorities {
        let _kp = keys::load_or_create_keypair(id);
        auth_map.insert(id.to_string(), role.to_string());
    }

    let txt = serde_json::to_string_pretty(&auth_map).unwrap();
    std::fs::write("keys/authorities.json", txt).unwrap();

    // -------- Target SAME Devices --------
    let devices = vec!["sim-device-001", "sim-device-002"];

    for dev in devices {

        for seq in 1..=commands_per_device {

            for (authority_id, _) in &authorities {

                let kp = keys::load_or_create_keypair(authority_id);

                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let action = match seq % 2 {
                    0 => SatelliteCommandType::ExecuteBurn { delta_v: 2.0 },
                    _ => SatelliteCommandType::Reset,
                };

                let cmd = Command {
                    authority_id: authority_id.to_string(),
                    device_id: dev.to_string(),
                    sequence: seq as u64,
                    timestamp: now,
                    expires_at: now + 30,
                    action,
                };

                let canonical = serde_json::to_vec(&cmd).unwrap();
                let sig_bytes =
                    crate::security::verify::sign(&kp.secret, &canonical);

                let signed = SignedCommand {
                    command: canonical,
                    signature: STANDARD.encode(&sig_bytes),
                };

                let _ = client::send_signed_command(addr, &signed);

                thread::sleep(Duration::from_millis(delay_ms));
            }
        }
    }
}

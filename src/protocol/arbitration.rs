use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration};
use std::collections::{HashSet, HashMap};

use crate::protocol::queue::QueuedCommand;
use crate::protocol::state::{DeviceState, SatelliteMode};
use crate::protocol::command::SatelliteCommandType;

pub fn start_arbitration_loop(
    queue: Arc<Mutex<Vec<QueuedCommand>>>,
    state: Arc<Mutex<DeviceState>>,
) {
    thread::spawn(move || {
        let mut finalized_sequences: HashMap<String, u64> = HashMap::new();

        loop {
            thread::sleep(Duration::from_millis(2000));

            let mut locked = queue.lock().unwrap();
            if locked.is_empty() {
                continue;
            }

            let mut snapshot = locked.clone();
            locked.clear();
            drop(locked);

            // Deterministic ordering
            snapshot.sort_by(|a, b| {
                a.command.timestamp
                    .cmp(&b.command.timestamp)
                    .then(a.command.sequence.cmp(&b.command.sequence))
            });

            let mut seen_in_batch: HashSet<(String, u64)> = HashSet::new();

            for entry in snapshot {
                let device = entry.command.device_id.clone();
                let seq = entry.command.sequence;

                // Skip duplicates inside same batch
                if seen_in_batch.contains(&(device.clone(), seq)) {
                    println!(
                        "[ARBITRATION] SKIPPED duplicate device={} seq={}",
                        device, seq
                    );
                    continue;
                }

                seen_in_batch.insert((device.clone(), seq));

                // Enforce strictly increasing sequence per device
                let last_seq = finalized_sequences.get(&device).cloned().unwrap_or(0);

                if seq != last_seq + 1 {
                    println!(
                        "[ARBITRATION] SKIPPED out-of-order device={} seq={}",
                        device, seq
                    );
                    continue;
                }

                // Apply real state transition
                {
                    let mut st = state.lock().unwrap();

                    match entry.command.action {
                        SatelliteCommandType::SetMode { ref mode } => {
                            match mode.as_str() {
                                "Idle" => st.set_mode(&device, SatelliteMode::Idle),
                                "SafeMode" => st.set_mode(&device, SatelliteMode::SafeMode),
                                _ => {}
                            }
                        }

                        SatelliteCommandType::ExecuteBurn { .. } => {
                            st.set_mode(&device, SatelliteMode::Maneuvering);
                        }

                        SatelliteCommandType::DeployPayload => {
                            st.set_mode(&device, SatelliteMode::PayloadDeployed);
                        }

                        SatelliteCommandType::Reset => {
                            st.set_mode(&device, SatelliteMode::Idle);
                        }
                    }
                }

                finalized_sequences.insert(device.clone(), seq);

                println!(
                    "[ARBITRATION] FINALIZED device={} seq={}",
                    device, seq
                );
            }
        }
    });
}

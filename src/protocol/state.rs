use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SatelliteMode {
    Idle,
    Maneuvering,
    SafeMode,
    PayloadDeployed,
}

pub struct DeviceState {
    pub sequences: HashMap<String, u64>,
    pub modes: HashMap<String, SatelliteMode>,
}

impl DeviceState {
    pub fn new() -> Self {
        Self {
            sequences: HashMap::new(),
            modes: HashMap::new(),
        }
    }

    pub fn validate_sequence(&mut self, device_id: &str, seq: u64) -> bool {
        let last = self.sequences.entry(device_id.to_string()).or_insert(0);
        if seq <= *last {
            return false;
        }
        *last = seq;
        true
    }

    pub fn get_mode(&self, device_id: &str) -> SatelliteMode {
        self.modes
            .get(device_id)
            .cloned()
            .unwrap_or(SatelliteMode::Idle)
    }

    pub fn set_mode(&mut self, device_id: &str, mode: SatelliteMode) {
        self.modes.insert(device_id.to_string(), mode);
    }
}

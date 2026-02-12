#[derive(Debug)]
pub struct PolicyContext {
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub network_latency_ms: u64,
    pub security_level: u8, // 1 = low, 2 = medium, 3 = high
}

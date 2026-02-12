pub struct Config {
    pub security_level: u8,
}
impl Config {
    pub fn load() -> Self {
        Self { security_level: 7 }
    }
}

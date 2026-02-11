use crate::monitoring::Metrics;

#[derive(Clone)]
pub enum SignatureAlgorithm {
    Dilithium,
    Falcon,
    Sphincs,
}

pub fn select(metrics: &Metrics, security: u8) -> SignatureAlgorithm {
    if security >= 9 {
        return SignatureAlgorithm::Sphincs;
    }
    if metrics.cpu < 60.0 {
        SignatureAlgorithm::Dilithium
    } else {
        SignatureAlgorithm::Falcon
    }
}

use crate::signature::SignatureAlgorithm;
use super::policy::SignaturePolicy;
use super::context::PolicyContext;

pub struct DefaultPolicy;

impl SignaturePolicy for DefaultPolicy {
    fn select_algorithm(&self, ctx: &PolicyContext) -> SignatureAlgorithm {

        if ctx.security_level == 3 {
            return SignatureAlgorithm::Sphincs;
        }

        if ctx.cpu_usage < 30.0 {
            SignatureAlgorithm::Dilithium
        } else if ctx.cpu_usage < 60.0 {
            SignatureAlgorithm::Falcon
        } else {
            SignatureAlgorithm::Sphincs
        }
    }
}

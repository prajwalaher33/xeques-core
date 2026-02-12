use crate::signature::SignatureAlgorithm;
use super::context::PolicyContext;

pub trait SignaturePolicy {
    fn select_algorithm(&self, ctx: &PolicyContext) -> SignatureAlgorithm;
}

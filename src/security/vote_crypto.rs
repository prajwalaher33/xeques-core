use pqcrypto_dilithium::dilithium5::{
    detached_sign,
    verify_detached_signature,
    DetachedSignature,
    PublicKey,
    SecretKey,
};

use pqcrypto_traits::sign::DetachedSignature as DetachedSignatureTrait;

pub fn sign_vote(sk: &SecretKey, payload: &[u8]) -> Vec<u8> {
    let sig = detached_sign(payload, sk);
    sig.as_bytes().to_vec()
}

pub fn verify_vote(pk: &PublicKey, payload: &[u8], sig: &[u8]) -> bool {

    let signature = match DetachedSignature::from_bytes(sig) {
        Ok(s) => s,
        Err(_) => return false,
    };

    verify_detached_signature(&signature, payload, pk).is_ok()
}

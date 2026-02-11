use pqcrypto_sphincsplus::sphincssha2128fsimple;
use pqcrypto_traits::sign::DetachedSignature;

/// Sign data with SPHINCS+ (shake-128f simple variant).
/// Returns hex-encoded detached signature.
pub fn sign(data: &[u8]) -> String {
    // generate ephemeral keypair for now (matches previous project pattern)
    let (_pk, sk) = sphincssha2128fsimple::keypair();
    let sig = sphincssha2128fsimple::detached_sign(data, &sk);
    hex::encode(sig.as_bytes())
}

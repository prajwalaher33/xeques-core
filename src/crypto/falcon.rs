use pqcrypto_falcon::falcon512;
use pqcrypto_traits::sign::DetachedSignature;

pub fn sign(data: &[u8]) -> String {
    let (_pk, sk) = falcon512::keypair();
    let sig = falcon512::detached_sign(data, &sk);
    hex::encode(sig.as_bytes())
}

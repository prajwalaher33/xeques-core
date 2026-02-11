use pqcrypto_dilithium::dilithium5;
use pqcrypto_traits::sign::DetachedSignature;

pub fn sign(data: &[u8]) -> String {
    let (_pk, sk) = dilithium5::keypair();
    let sig = dilithium5::detached_sign(data, &sk);
    hex::encode(sig.as_bytes())
}

use ring::hmac::{Key, HMAC_SHA256, Tag, sign, verify};
use ring::rand::SystemRandom;
use ring::rand::SecureRandom;
use ring::error::Unspecified;
use ring::{
    rand,
    signature::{self, KeyPair, Ed25519KeyPair, Signature},
};

pub fn generate_keys() -> std::result::Result<Ed25519KeyPair, Unspecified> {
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())?;
    Ok(key_pair)
}

pub fn sign_data(key_pair: Ed25519KeyPair, data: &[u8]) -> Signature {
    key_pair.sign(data)
}

pub fn verify_data(public_key_bytes: &[u8], data: &[u8], signature: Signature) -> bool {
    let peer_public_key = signature::UnparsedPublicKey::new(&signature::ED25519, public_key_bytes);
    match peer_public_key.verify(data, signature.as_ref()) {
        Ok(()) => { true },
        _ => { false }
    }
}

// TODO: Add new tests
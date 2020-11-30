use ring::hmac::{Key, HMAC_SHA256, Tag, sign, verify};
use ring::rand::SystemRandom;
use ring::rand::SecureRandom;
use ring::error::Unspecified;

pub fn generate_key() -> Result<Key, Unspecified> {
    let mut key_val = [0u8; 48];
    let rng = SystemRandom::new();
    rng.fill(&mut key_val)?;

    Ok(Key::new(HMAC_SHA256, &key_val))
}

pub fn sign_data(key: &Key, data : &[u8]) -> Tag {
    sign(&key, &data)
}

pub fn verify_data(key: &Key, data: &[u8], signature: &[u8]) -> Result<(), Unspecified> {
    verify(&key, &data, signature)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use ring::hmac::Tag;
    use super::*;

    #[test]
    fn key_generation_tests() {
        for _ in 0..50 {
            generate_key().unwrap();
        }
    }

    #[test]
    fn data_signing_test() {
        let test_strings : [&str; 5] = [
            "Test",
            "Hello World",
            "Hashing is fun",
            "Decentralization is the future",
            "a-?.123'"
        ];

        for string in &test_strings {
            match generate_key() {
                Ok(new_key) => {
                    let key = new_key;
                    sign_data(&key, &string.as_bytes());
                },
                _ => { panic!(); }
            };
       }
    }

    #[test]
    fn data_vertification_test() {
        let test_strings : [&str; 5] = [
            "Test",
            "Hello World",
            "Hashing is fun",
            "Decentralization is the future",
            "a-?.123'"
        ];

        for string in &test_strings {
            match generate_key() {
                Ok(new_key) => {
                    let key = new_key;
                    let signature: Tag = sign_data(&key, &string.as_bytes());
                    verify_data(&key, &string.as_bytes(), signature.as_ref()).unwrap();
                },
                _ => { panic!(); }
            };
       }
    }

    #[test]
    fn data_vertification_failure_test() {
        // TODO
    }
}
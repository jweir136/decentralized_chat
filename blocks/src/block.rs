use std::time::Instant;
use ring::hmac::{Key, Tag};
use ring::digest::{Digest, Context, SHA256};
use ring::signature::{Ed25519KeyPair, Signature};

use block_cryptography::hashing::hash_digest;
use block_cryptography::digital_signing::{sign_data, verify_data};

trait Block {
    fn is_correct(&self, public_key: &[u8]) -> bool;
}

struct Chatblock {
    to: String,
    from: String,
    time: Instant,
    msg: String,
    hash: Option<Digest>,
    last_hash: Option<Digest>,
    signature: Option<Signature>,
    pos: usize
}

impl std::io::Read for Chatblock {
    fn read(&mut self, buff: &mut [u8]) -> std::io::Result<usize> {
        let mut n_read: usize = 0;
        let string: &str = &format!("{}{}{}{:?}", self.to, self.from, self.msg, self.time);

        for i in self.pos..buff.len() {
            buff[i] = string.as_bytes()[self.pos + i];
            n_read += 1;
        }

        self.pos += n_read;
        Ok(n_read)
    }
}

impl Chatblock {
    pub fn new(to: String, from: String, msg: String, last_hash: Digest, keypair: &Ed25519KeyPair) -> Self {
        let mut block: Self = Chatblock {
            to: to,
            from: from,
            time: Instant::now(),
            msg: msg,
            hash: Option::None,
            last_hash: Option::Some(last_hash),
            signature: Option::None,
            pos: 0
        };

        match hash_digest(&mut block, Context::new(&SHA256)) {
            Ok(digest) => { block.hash = Option::Some(digest); },
            _ => { panic!("Error: Cannot hash block"); }
        };
        block.signature = Option::Some(sign_data(&keypair, &format!("{:?}", block.hash).as_bytes()));
        block
    }
}

impl Block for Chatblock {
    fn is_correct(&self, public_key: &[u8]) -> bool {
        verify_data(public_key, &format!("{:?}", self.hash).as_bytes(), self.signature.unwrap())
    }
}

struct Nullblock {
    hash: Option<Digest>,
    last_hash: Digest,
    pos: usize
}

impl std::io::Read for Nullblock {
    fn read(&mut self, buff: &mut [u8]) -> std::io::Result<usize> {
        let mut n_read: usize = 0;
        let string: &str = "NULL";

        for i in self.pos..buff.len() {
            buff[i] = string.as_bytes()[self.pos + i];
            n_read += 1;
        }

        self.pos += n_read;
        Ok(n_read)
    }
}

impl Nullblock {
    pub fn new(last_hash: Digest) -> Self {
        let mut block: Nullblock = Nullblock {
            hash: Option::None,
            last_hash: last_hash,
            pos: 0
        };
        block.hash = Option::Some(hash_digest(&mut block, Context::new(&SHA256)).unwrap());
        block
    }
}

impl Block for Nullblock {
    fn is_correct(&self, public_key: &[u8]) -> bool {
        true
    }
}
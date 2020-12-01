use std::io::{Read, Result};
use ring::digest::Digest;
use ring::hmac::Tag;
use std::time::Instant;

use block_cryptography::hash_digest;

trait Block : Read + Copy {
    fn hash(&self) -> Result<Digest>;

    fn last_hash(&self) -> Result<Digest>;
}

pub struct Chatblock {
    to: String,
    from: String,
    time: Instant,
    msg: String,
    hash: Digest,
    last_hash: Digest,
    signature:  Tag,
    position: usize
}

pub struct Nullblock {
    hash: Digest,
    last_hash: Digest
}

pub struct Genesisblock {
    hash: Digest,
    last_hash: Digest
}

impl Read for Chatblock {
    fn read(&mut self, buff: &mut [u8]) -> Result<usize> {
        let mut bytes_read: usize = 0;
        let string: String = format!("{:?}{:?}", self.hash, self.last_hash);
        
        for i in self.position..buff.len() {
            buff[i] = (&string).as_bytes()[self.position + i];
            bytes_read += 1;
        }

        self.position += bytes_read;
        Ok(bytes_read)
    }
}

impl Chatblock {
    pub fn new(to: String, from: String, msg: String) -> Self {
        // todo
    }
}
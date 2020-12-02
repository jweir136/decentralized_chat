use crate::block::{Block};

pub trait Blockchain {
    fn add<B: Block>(&mut self, block: B);
    fn delete(&mut self, key: usize) -> std::io::Result<()>;
    fn get(&mut self, key: usize) -> Option<Box<dyn Block>>;
}
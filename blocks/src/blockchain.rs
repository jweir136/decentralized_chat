use crate::block::{Block, Chatblock};

pub trait Blockchain {
    fn add(&mut self, block: Box<dyn Block>);
    fn get(&mut self, key: usize) -> Option<Box<dyn Block>>;
}
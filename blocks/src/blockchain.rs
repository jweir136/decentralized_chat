use crate::block::{Block, Chatblock};

pub trait Blockchain {
    fn add<B: Block>(&mut self, block: B);
    fn get(&mut self, key: usize) -> Option<Box<dyn Block>>;
}

pub struct VectorBlockchain {
    blocks: Vec<Chatblock>
}

impl VectorBlockchain {
    pub fn new() -> Self {
        VectorBlockchain { blocks: Vec::<Chatblock>::new() }
    }
}

impl Blockchain for VectorBlockchain {
    fn add<B: Block>(&mut self, block: B) {
        self.blocks.push(block);
    }

    fn get(&mut self, key: usize) -> Option<Box<dyn Block>> {
        self.blocks[key]
    }
}
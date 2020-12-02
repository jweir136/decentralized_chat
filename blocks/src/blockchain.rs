use crate::block::{Block};

pub trait Blockchain {
    fn add(&mut self, block: Box<dyn Block>);
    fn get(&mut self, key: usize) -> Option<&Box<dyn Block>>;
}

struct VectorBlockchain {
    blocks: Vec<Box<dyn Block>>
}

impl VectorBlockchain {
    pub fn new() -> Self {
        VectorBlockchain {
            blocks: Vec::<Box<dyn Block>>::new()
        }
    }
}

impl Blockchain for VectorBlockchain {
    fn add(&mut self, block: Box<dyn Block>) {
        self.blocks.push(block);
    }

    fn get(&mut self, key: usize) -> Option<&Box<dyn Block>> {
        if key >= self.blocks.len() {
            Option::None
        } else {
            Option::Some(&self.blocks[key])
        }
    }
}
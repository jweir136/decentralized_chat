use crate::block::Block;

struct Blockchain {
    blocks: Vec<Box<Block>>,
    pos: usize
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: Vec::<Box<Block>>::new(),
            pos: 0
        }
    }

    pub fn add(&mut self, block: Box<Block>) {
        self.blocks.push(block);
    }
}
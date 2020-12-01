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

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn get(&self, idx: usize) -> Option<&Box<Block>> {
        if idx >= self.blocks.len() {
            Option::None
        } else {
            Option::Some(&self.blocks[idx])
        }
    }
}
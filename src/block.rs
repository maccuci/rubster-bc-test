use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use chrono::{Duration, Utc};

pub struct Block {
    pub index: u32,
    pub created_time: Duration,
    pub data: String,
    pub hash_previous: u64,
    pub hash: u64
}

pub struct Blockchain {
    pub blocks: Vec<Block>
}

impl Block {
    pub fn calc_hash(&self) -> u64 {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        self.hash_previous.hash(&mut hasher);
        self.data.hash(&mut hasher);
        self.created_time.to_std().unwrap().hash(&mut hasher);
        hasher.finish()
    }

    pub fn create_block(index: u32, data: String, hash_previous: u64) -> Block {
        let created_time: i64 = Utc::now().timestamp();
        let hash: u64 = Block::calc_hash(&Block {
            index,
            created_time: Duration::zero() + Duration::seconds(created_time),
            data: data.clone(),
            hash_previous,
            hash: 0,
        });
        Block {
            index,
            created_time: Duration::zero() + Duration::seconds(created_time),
            data,
            hash_previous,
            hash,
        }
    }
}

impl Blockchain {
    pub fn invoke(index: u32, data: String, hash_previous: u64) -> Blockchain {
        let item: Block = Block::create_block(index, data, hash_previous);
        Blockchain { blocks: vec![item] }
    }

    pub fn push(&mut self, block: Block) {
        self.blocks.push(block);
    }
}
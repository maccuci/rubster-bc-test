use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    #[serde(with = "serde_duration")]
    pub created_time: Duration,
    pub data: String,
    pub hash_previous: u64,
    pub hash: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>
}

mod serde_duration {
    use serde::{Serializer, Deserializer, Deserialize};
    use chrono::Duration;

    pub fn serialize<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_i64(value.num_seconds())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
        where
            D: Deserializer<'de>,
    {
        let seconds = i64::deserialize(deserializer)?;
        Ok(Duration::seconds(seconds))
    }
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
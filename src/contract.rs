use serde::{Deserialize, Serialize};
use crate::block::{Block, Blockchain};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    id: i8,
    pub balance: u64,
    pub blockchains: Vec<Block>
}

impl Contract {
    pub fn create(id: i8) -> Contract {
        Contract { id, balance: 0, blockchains: Vec::new() }
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }

    pub fn deposit(&mut self, value: u64) {
        self.balance += value;
    }

    pub fn withdraw(&mut self, value: u64) -> Result<(), &'static str> {
        if value < 1  {
            return Err("Value cannot be 0.");
        }
        if value > self.balance {
            return Err("Error: Insufficient balance.")
        }
        self.balance -= value;
        Ok(())
    }

    pub fn add_blockchain_item(&mut self, blockchain: Blockchain) {
        for block in &blockchain.blocks {
            if self.blockchains.contains(block) {
                panic!("The blockchain index {} already exists in the contract.", block.index);
            }
            self.blockchains.push(
                Block {
                    index: block.index,
                    created_time: block.created_time,
                    data: block.data.clone(),
                    hash_previous: block.hash_previous,
                    hash: block.hash
                }
            );
        }
    }
}
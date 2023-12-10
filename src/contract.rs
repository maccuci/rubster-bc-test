use serde::{Deserialize, Serialize};
use crate::block::{Block, Blockchain};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    pub balance: u64,
    pub blockchains: Vec<Block>
}

impl Contract {
    pub fn create() -> Contract {
        Contract { balance: 0, blockchains: Vec::new() }
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }

    pub fn deposit(&mut self, value: u64) {
        self.balance += value;
    }

    pub fn withdraw(&mut self, value: u64) -> Result<(), &'static str> {
        if value > self.balance {
            Err("Your balance is minor.")
        } else {
        self.balance -= value;
        Ok(())
        }
    }

    pub fn add_blockchain_item(&mut self, blockchain: Blockchain) {
        for block in &blockchain.blocks {
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
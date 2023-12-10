use crate::block::{Block, Blockchain};

mod block;

fn main() {
    let mut chain: Blockchain = Blockchain::invoke(0, "Argona".to_string(), 0);

    /// How to add a new item to a blockchain.
    // let blockchain = Block::create_block(1, "Example".to_string(), chain.blocks.last().unwrap().hash);
    // chain.push(blockchain);

    for block in &chain.blocks {
        print!(
            "Blockchain Collection - {}\nID: #{} - Hash: {} - Hash Previous: {}\n",
            block.data.to_uppercase(), block.index, block.hash, block.hash_previous
        )
    }
}

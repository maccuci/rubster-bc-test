use crate::block::{Block, Blockchain};
use crate::contract::Contract;

mod block;
mod contract;

fn main() {
    let mut chain: Blockchain = Blockchain::invoke(0, "Argona".to_string(), 0);

    /// How to add a new item to a blockchain.
    let blockchain = Block::create_block(1, "Example".to_string(), chain.blocks.last().unwrap().hash);
    chain.push(blockchain);

    for block in &chain.blocks {
        print!(
            "Blockchain Collection - {}\nID: #{} - Hash: {} - Hash Previous: {}\n",
            block.data.to_uppercase(), block.index, block.hash, block.hash_previous
        )
    }

    let mut contract = Contract::create(1);

    contract.add_blockchain_item(chain);
    contract.deposit(100);
    println!("Your balance: {}", contract.get_balance());

    if let Err(err) = contract.withdraw(1) {
        println!("Error: {}", err);
        return;
    }
    println!("Your new balance is: {}", contract.get_balance());
    for block in contract.blockchains {
        print!(
            "Blockchain Collection - {}\nID: #{} - Hash: {} - Hash Previous: {}\n",
            block.data.to_uppercase(),
            block.index,
            block.hash,
            block.hash_previous
        );
    }
}

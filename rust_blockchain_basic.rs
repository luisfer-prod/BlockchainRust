use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = current_timestamp();
        let (nonce, hash) = mine_block(index, timestamp, &data, &previous_hash);
        Block { index, timestamp, data, previous_hash, hash, nonce }
    }
}

fn current_timestamp() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

fn calculate_hash(index: u64, timestamp: u128, data: &str, previous_hash: &str, nonce: u64) -> String {
    let input = format!("{}{}{}{}{}", index, timestamp, data, previous_hash, nonce);
    let mut hasher = Sha256::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}

fn mine_block(index: u64, timestamp: u128, data: &str, previous_hash: &str) -> (u64, String) {
    let mut nonce = 0;
    loop {
        let hash = calculate_hash(index, timestamp, data, previous_hash, nonce);
        if hash.starts_with("0000") {
            return (nonce, hash);
        }
        nonce += 1;
    }
}

struct Blockchain {
    chain: VecDeque<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            chain: VecDeque::from(vec![genesis])
        }
    }

    fn add_block(&mut self, data: String) {
        let last_block = self.chain.back().unwrap();
        let new_block = Block::new(last_block.index + 1, data, last_block.hash.clone());
        self.chain.push_back(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }

            let recalculated_hash = calculate_hash(
                current.index,
                current.timestamp,
                &current.data,
                &current.previous_hash,
                current.nonce
            );

            if current.hash != recalculated_hash {
                return false;
            }
        }
        true
    }

    fn print_chain(&self) {
        for block in &self.chain {
            println!("{:?}\n", block);
        }
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("Transaction 1: A -> B".to_string());
    blockchain.add_block("Transaction 2: B -> C".to_string());
    blockchain.add_block("Transaction 3: C -> D".to_string());

    blockchain.print_chain();
    println!("Is blockchain valid? {}", blockchain.is_valid());
}

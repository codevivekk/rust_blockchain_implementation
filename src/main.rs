use chrono::Utc;
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Block {
    hash: String,
    data: String,
    index: u32,
    previous_hash: String,
    timestamp: String,
}

struct Blockchain {
    chain: Vec<Block>,
}

fn calculate_hash(format_string: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format_string);
    let hash_result = hasher.finalize();
    format!("{:x}", hash_result)
}

impl Blockchain {
    fn genesis_block() -> Self {
        let genesis_block = Block::new_block(1, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: String) {
        let last_block = self
            .chain
            .last()
            .expect("Blockchain Should have atleast one block");
        let new_block = Block::new_block(last_block.index + 1, data, last_block.hash.clone());
        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            let format_string = format!(
                "{}{}{}{}",
                current_block.index,
                current_block.timestamp,
                current_block.data,
                current_block.previous_hash
            );

            let calculated_hash = calculate_hash(&format_string);

            if current_block.hash != calculated_hash {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }

        true
    }
}

impl Block {
    fn new_block(index: u32, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let format_string = format!("{}{}{}{}", index, timestamp, data, previous_hash);
        let hash = calculate_hash(&format_string);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

fn main() {
    let mut blockchain = Blockchain::genesis_block();
    blockchain.add_block("My Name is Vivek Kushwaha".to_string());
    blockchain.add_block("My Name is Vivek Kushwaha 2".to_string());

    if !blockchain.is_valid() {
        println!("Blockchain is invalid");
        return;
    }

    for block in blockchain.chain.iter() {
        println!("{:#?}", block);
    }
}

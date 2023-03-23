use crate::transaction::Transaction;
use crate::block::Block;
use crate::pow::{proof_of_work, is_valid_proof};
use crate::utils::hash_block;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: 0,
            transactions: vec![],
            proof: 0,
            previous_hash: String::from("GENESIS"),
        };

        Self {
            chain: vec![genesis_block],
            pending_transactions: vec![],
        }
    }

    pub fn add_transaction(&mut self, sender: String, recipient: String, amount: f64) {
        self.pending_transactions.push(Transaction {
            sender,
            recipient,
            amount,
        });
    }

    pub fn add_block(&mut self, proof: u64) {
        let last_block = self.chain.last().unwrap().clone();
        let block = Block {
            index: last_block.index + 1,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            transactions: self.pending_transactions.clone(),
            proof,
            previous_hash: hash_block(&last_block),
        };
        self.chain.push(block);
        self.pending_transactions.clear();
    }

    pub fn proof_of_work(&self, last_proof: u64) -> u64 {
        proof_of_work(last_proof)
    }

    pub fn is_valid_proof(&self, last_proof: u64, proof: u64) -> bool {
        is_valid_proof(last_proof, proof)
    }
}


use crate::block::Block;
use serde_json::json;

pub fn hash_block(block: &Block) -> String {
    let block_data = json!({
        "index": block.index,
        "timestamp": block.timestamp,
        "transactions": block.transactions,
        "proof": block.proof,
        "previous_hash": block.previous_hash,
    });

    let serialized_block = serde_json::to_string(&block_data).unwrap();
    crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, serialized_block.as_bytes())
}


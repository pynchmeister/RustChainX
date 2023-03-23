use std::fmt::{Debug, Formatter};

const DIFFICULTY_PREFIX: &str = "0000";

pub fn proof_of_work(last_proof: u64) -> u64 {
    let mut proof = 0;
    while !is_valid_proof(last_proof, proof) {
        proof += 1;
    }
    proof
}

pub fn is_valid_proof(last_proof: u64, proof: u64) -> bool {
    let guess = format!("{}{}", last_proof, proof);
    let guess_hash = crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, guess.as_bytes());
    guess_hash.starts_with(DIFFICULTY_PREFIX)
}

impl Debug for ProofOfWork {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProofOfWork")
    }
}


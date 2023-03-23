use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use warp::{Rejection, Reply};

pub async fn handle_get_chain(
    blockchain: Blockchain,
) -> Result<impl Reply, Rejection> {
    let chain_json = serde_json::to_string(&blockchain.chain).unwrap();
    Ok(warp::reply::json(&chain_json))
}

pub async fn handle_create_transaction(
    transaction: Transaction,
    mut blockchain: Blockchain,
) -> Result<impl Reply, Rejection> {
    blockchain.add_transaction(transaction.sender, transaction.recipient, transaction.amount);
    Ok(warp::reply::with_status(
        "Transaction added to pending transactions",
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn handle_mine_block(
    mut blockchain: Blockchain,
) -> Result<impl Reply, Rejection> {
    let last_block = blockchain.chain.last().unwrap();
    let last_proof = last_block.proof;
    let proof = blockchain.proof_of_work(last_proof);

    blockchain.add_transaction("0".to_string(), "miner_address".to_string(), 1.0);
    blockchain.add_block(proof);

    Ok(warp::reply::with_status(
        "New block mined and added to the blockchain",
        warp::http::StatusCode::CREATED,
    ))
}


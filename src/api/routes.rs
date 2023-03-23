use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use warp::{Filter, Reply};

pub fn routes(
    blockchain: Blockchain,
) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let blockchain_ref = warp::any().map(move || blockchain.clone());

    let get_chain = warp::path("chain")
        .and(warp::get())
        .and(blockchain_ref.clone())
        .and_then(crate::api::handlers::handle_get_chain);

    let create_transaction = warp::path("transaction")
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_ref.clone())
        .and_then(crate::api::handlers::handle_create_transaction);

    let mine_block = warp::path("mine")
        .and(warp::post())
        .and(blockchain_ref.clone())
        .and_then(crate::api::handlers::handle_mine_block);

    get_chain.or(create_transaction).or(mine_block)
}


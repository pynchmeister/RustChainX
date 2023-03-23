mod blockchain;
mod transaction;
mod block;
mod pow;
mod utils;
mod api;

use api::routes;
use warp::{Filter, Reply};

#[tokio::main]
async fn main() {
    let blockchain = blockchain::Blockchain::new();
    let blockchain_ref = warp::any().map(move || blockchain.clone());

    let get_chain = warp::path("chain")
        .and(warp::get())
        .and(blockchain_ref.clone())
        .and_then(api::handlers::handle_get_chain);

    let create_transaction = warp::path("transaction")
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_ref.clone())
        .and_then(api::handlers::handle_create_transaction);

    let mine_block = warp::path("mine")
        .and(warp::post())
        .and(blockchain_ref.clone())
        .and_then(api::handlers::handle_mine_block);

    let routes = get_chain.or(create_transaction).or(mine_block);

    println!("Starting server on 127.0.0.1:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}


use std::collections::HashMap;
use std::sync::Arc;
use axum::{Extension, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use std::env;
use axum::{routing::get, extract::Json, extract::Path};

type SharedMap = std::sync::Arc<tokio::sync::Mutex<HashMap<String, Vec<Transaction>>>>;

use edblock::{blockchain::{blockchain_core::Transaction, peer_network::Node}, utils::get_value};

#[tokio::main]
async fn main() {

    let transaction_map = Arc::new(tokio::sync::Mutex::new(HashMap::<String, Vec<Transaction>>::new()));

    let port: u16 = get_value("Enter port number: ").parse().unwrap();
    let exp_node = Node::new(port, "0.0.0.0:1234".to_string()).await;

    let node_address = get_value("Enter any node address: ");
    exp_node.add_peer(node_address).await;

    let transaction_map_clone = transaction_map.clone();
    tokio::spawn(async move {
        while let Ok(msg) = exp_node.msg_incoming_rx.recv() {
            if let Some(block) = msg.block {
                for transaction in block.transactions.values() {
                    println!("New certificate: {}", transaction.certificate);
                    let stud_addr = &transaction.certificate.stud_wallet_addr;
                    let uni_addr = &transaction.certificate.uni_wallet_addr;

                    if !stud_addr.is_empty() {
                        let mut transaction_map = transaction_map_clone.lock().await;
                        transaction_map.entry(stud_addr.clone())
                            .or_insert_with(Vec::new)
                            .push(transaction.clone());
                    }

                    if !uni_addr.is_empty() {
                        let mut transaction_map = transaction_map_clone.lock().await;
                        transaction_map.entry(uni_addr.clone())
                            .or_insert_with(Vec::new)
                            .push(transaction.clone());
                    }
                }
            }
        }
    });

    explorer_app_run(transaction_map, port).await.unwrap();
}

pub async fn explorer_app_run(transaction_map: SharedMap, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{}=debug, tower_http=debug", env!("CARGO_CRATE_NAME")).into()
        }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    tokio::join!(
        serve(api_end_points(transaction_map), port),
    );
    Ok(())
}

fn api_end_points(transaction_map: SharedMap) -> Router {
    Router::new()
    .route("/get_transaction/:address", get(get_transaction))
    .layer(Extension(transaction_map))
}

async fn get_transaction(Path(address): Path<String>, Extension(trasaction_map): Extension<SharedMap>) -> Json<Vec<Transaction>> {
    let transaction_map = trasaction_map.lock().await;
    if let Some(trans_vec) = transaction_map.get(&address) {
        Json(trans_vec.to_vec())
    }else {
        Json(Vec::new())
    }
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([0,0,0,0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listeneing on {}",listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}
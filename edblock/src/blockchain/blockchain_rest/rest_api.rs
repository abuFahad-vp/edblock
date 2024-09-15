use axum::response::Html;
use axum::routing::post;
use axum::{Extension, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::{net::SocketAddr, process::Command};
use tower_http::{services::ServeDir, trace::TraceLayer};
use std::env;
use axum::{routing::get, extract::Json};

use crate::blockchain::wallet::KeyPairAddress;
use crate::blockchain::SharedChain;
use crate::blockchain::blockchain_core::Certificate;

use crate::rust_template::key_generator_template;

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct Len {
    pub uuid: String,
    pub len: u32,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct Msg {
    pub uuid: String,
    pub status: u32,
    pub notes: String,
}

pub async fn blockchain_app_run(chain: SharedChain, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{}=debug, tower_http=debug", env!("CARGO_CRATE_NAME")).into()
        }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    tokio::join!(
        serve(api_end_points(chain), port),
    );
    Ok(())
}

fn api_end_points(chain: SharedChain) -> Router {
    Router::new()
    .route("/archive_db", get(make_archive))
    .route("/len", get(get_len))
    .route("/generate_key", get(key_generator))
    .route("/transaction", post(transaction))
    .nest_service("/static", ServeDir::new("static"))
    .layer(Extension(chain))
}

async fn key_generator() -> Html<String> {
    let keys = KeyPairAddress::new();
    Html(key_generator_template::key_pair_template(keys))
}

async fn get_len(Extension(chain): Extension<SharedChain>) -> Json<Len> {
    let mut chain = chain.lock().await;
    Json(Len {uuid: chain.node.get_id().to_string(), len: chain.get_height().await})
}

async fn transaction(chain: Extension<SharedChain>, Json(form_data): Json<Certificate>) -> Json<Msg> {
    let mut chain = chain.lock().await;
    println!("New transaction of certificate: {}", form_data);
    let status = chain.new_transaction(form_data);
    if status {
        println!("New transaction successfully added");
        Json(Msg{uuid: chain.node.get_id().to_string(), status: 200, notes: "Successfully added".to_string()})
    } else {
        Json(Msg{uuid: chain.node.get_id().to_string(), status: 400, notes: "Failed to add the transaction".to_string()})
    }
} 

// make an archive of db and copy to static folder
async fn make_archive(chain: Extension<SharedChain>) -> Json<Msg>{

    let mut chain = chain.lock().await;
    chain.copy_db_backup().await;
    let current_dir = env::current_dir().unwrap();
    let file_dir = format!("{}\\backup\\",current_dir.display());
    if let Err(_) = Command::new("7z")
        .args(["a", "-tzip", "static/db.zip", &file_dir])
        .output() {
            Json(Msg {uuid: chain.node.get_id().to_string(), status: 400, notes: "Failed".to_string()})
    } else {
            Json(Msg {uuid: chain.node.get_id().to_string(), status: 200, notes: "Success".to_string()})
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
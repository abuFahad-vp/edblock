use clap::Parser;
use edblock::blockchain;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 8000, help = "port for running the node")]
    port: u32,
}

#[tokio::main]
async fn main() {
    if let Err(e) = std::fs::create_dir("static") {
        eprintln!("Couldn't able to create the static dir: {e}")
    }

    if let Err(e) = std::fs::create_dir("backup") {
        eprintln!("Couldn't able to create the backup dir: {e}")
    }
    blockchain::blockchain_app::blockchain_app().await;
    // let account = blockchain::wallet::KeyPairAddress::new();
    // println!("{account:?}");
}
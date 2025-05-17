// main.rs
use dotenvy::dotenv;
mod telegram;
mod db;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    db::init().await.expect("DB init failed");
    telegram::start_bot().await;
}
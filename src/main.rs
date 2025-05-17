use dotenvy::dotenv;
use teloxide::prelude::*;
mod db;
mod telegram;
mod services;
mod locales;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let pool = db::init().await.expect("DB init failed");
    db::queries::set_pool(pool);

    log::info!("🚀 Kalorik bot is running!");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(telegram::handlers::handle_message))
        .branch(Update::filter_callback_query().endpoint(telegram::handlers::handle_callback));

    Dispatcher::builder(bot, handler)
        .build()
        .dispatch()
        .await;
}

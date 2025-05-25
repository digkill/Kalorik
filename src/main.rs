use crate::telegram::handlers::{handle_callback, handle_message};
use teloxide::{
    dptree,
    prelude::*,
    types::{CallbackQuery, Message},
};
use actix_web::{App, HttpServer};

mod db;
mod locales;
mod services;
mod telegram;
mod webhook;
pub const URL_LINK_PAY: &'static str = env!("URL_LINK_PAY");

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    pretty_env_logger::init(); // Use only one logger initialization

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8282".into());
    let url_pay_link = std::env::var("URL_LINK_PAY").unwrap_or_else(|_| "http://localhost:8383".into());
    let addr = format!("{}:{}", host, port);

    // Initialize the database
    let pool = db::init()
        .await
        .expect("❌ Не удалось подключиться к базе данных");
    crate::db::queries::set_pool(pool);

    // Initialize the Telegram bot
    let bot = Bot::from_env();

    // Set up the dispatcher schema
    let schema = dptree::entry()
        .branch(
            Update::filter_message()
                .endpoint(|bot: Bot, msg: Message| async move { handle_message(bot, msg).await }),
        )
        .branch(
            Update::filter_callback_query().endpoint(|bot: Bot, q: CallbackQuery| async move {
                handle_callback(bot, q).await
            }),
        );

    // Start the dispatcher in a separate task
    tokio::spawn(async move {
        Dispatcher::builder(bot.clone(), schema)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    });

    println!("🚀 Webhook server running at http://{}", addr);

    // Start the Actix Web server
    HttpServer::new(|| App::new().service(webhook::subscription_callback))
        .bind(addr)?
        .run()
        .await
}
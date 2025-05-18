use teloxide::{prelude::*, types::{Message, CallbackQuery}, dptree};
use crate::telegram::handlers::{handle_message, handle_callback};

mod telegram;
mod db;
mod services;
mod locales;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    // Инициализация БД
    let pool = db::init().await.expect("❌ Не удалось подключиться к базе данных");
    crate::db::queries::set_pool(pool);

    // Telegram бот
    let bot = Bot::from_env();

    let schema = dptree::entry()
        .branch(
            Update::filter_message()
                .endpoint(|bot: Bot, msg: Message| async move {
                    handle_message(bot, msg).await
                }),
        )
        .branch(
            Update::filter_callback_query()
                .endpoint(|bot: Bot, q: CallbackQuery| async move {
                    handle_callback(bot, q).await
                }),
        );

    Dispatcher::builder(bot.clone(), schema)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

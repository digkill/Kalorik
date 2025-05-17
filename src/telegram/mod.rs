pub mod handlers;

use teloxide::prelude::*;
use crate::telegram::handlers::*;

pub async fn start_bot() {
    let bot = Bot::from_env();
    Dispatcher::builder(bot, Update::filter_message().endpoint(handle_message))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
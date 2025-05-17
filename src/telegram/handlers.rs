use teloxide::{prelude::*, types::Message};
use crate::db::queries;

pub async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    // 📩 Обработка текста
    if let Some(text) = msg.text() {
        if text == "/start" {
            queries::register_user(chat_id.0).await.ok();
            bot.send_message(chat_id, "🍽️ Добро пожаловать! Пиши, фотографируй или записывай аудио всё что ты ел — я всё посчитаю!").await?;
        } else {
            let result = crate::services::nutrition::analyze_food_description(text).await;
            bot.send_message(chat_id, result).await?;
        }
        return Ok(());
    }

    // Фото
    if let Some(photos) = msg.photo() {
        if let Some(photo) = photos.last() {
            let file_id = &photo.file.id;
            let file = bot.get_file(file_id).send().await?;
            let path = file.path;
            let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
            let url = format!("https://api.telegram.org/file/bot{}/{}", token, path);
            let result = crate::services::nutrition::analyze_image(&url).await;
            bot.send_message(chat_id, result).await?;
        }
        return Ok(());
    }

    // Голос
    if let Some(voice) = msg.voice() {
        let file_id = &voice.file.id;
        let file = bot.get_file(file_id).send().await?;
        let path = file.path;
        let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
        let url = format!("https://api.telegram.org/file/bot{}/{}", token, path);
        let result = crate::services::nutrition::analyze_audio(&url).await;
        bot.send_message(chat_id, result).await?;
        return Ok(());
    }

    // ℹ️ По умолчанию
    bot.send_message(chat_id, "Пожалуйста, отправь текст, фото еды или голосовое сообщение.").await?;
    Ok(())
}

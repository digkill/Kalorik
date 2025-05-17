use crate::db::queries;
use crate::locales::messages::Messages;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
};

pub async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let user_lang = "ru"; // TODO: заменить на реальное извлечение из пользователя
    let messages = Messages::get(user_lang);

    // Обработка текстовых сообщений
    if let Some(text) = msg.text() {
        if text == "/start" {
            queries::register_user(chat_id.0).await.ok();

            bot.send_message(chat_id, messages.welcome).await?;

            bot.send_message(
                chat_id,
                "🌐 Choose your language / Выберите язык / เลือกภาษา / 选择语言",
            )
            .reply_markup(InlineKeyboardMarkup::new([
                vec![
                    InlineKeyboardButton::callback("🇷🇺 Русский", "lang_ru"),
                    InlineKeyboardButton::callback("🇬🇧 English", "lang_en"),
                ],
                vec![
                    InlineKeyboardButton::callback("🇹🇭 ไทย", "lang_th"),
                    InlineKeyboardButton::callback("🇨🇳 中文", "lang_zh"),
                ],
            ]))
            .await?;
        } else {
            let result = crate::services::nutrition::analyze_food_description(text).await;
            bot.send_message(chat_id, result).await?;
        }

        if text == "/help" {
            bot.send_message(chat_id, messages.help.clone()).await?;
            return Ok(());
        }

        return Ok(());
    }

    // Обработка изображений
    if let Some(photos) = msg.photo() {
        if let Some(photo) = photos.last() {
            let file_id = &photo.file.id;
            let file = bot.get_file(file_id).send().await?;
            let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
            let url = format!("https://api.telegram.org/file/bot{}/{}", token, file.path);
            let result = crate::services::nutrition::analyze_image(&url).await;
            bot.send_message(chat_id, result).await?;
        }
        return Ok(());
    }

    // Обработка голосовых сообщений
    if let Some(voice) = msg.voice() {
        let file_id = &voice.file.id;
        let file = bot.get_file(file_id).send().await?;
        let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
        let url = format!("https://api.telegram.org/file/bot{}/{}", token, file.path);
        let result = crate::services::nutrition::analyze_audio(&url).await;
        bot.send_message(chat_id, result).await?;
        return Ok(());
    }

    // По умолчанию
    bot.send_message(chat_id, messages.unknown).await?;
    Ok(())
}

// Обработчик callback-запросов (в другом месте — обычно в dispatcher)
pub async fn handle_callback(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    let query = q; // по сути, уже callback_query

    if let Some(data) = query.data.as_deref() {
        let chat_id = query
            .message
            .as_ref()
            .map(|m| m.chat().id)
            .unwrap_or(ChatId(0));
        let lang_code = match data {
            "lang_ru" => "ru",
            "lang_en" => "en",
            "lang_th" => "th",
            "lang_zh" => "zh",
            _ => "ru",
        };

        queries::update_language(chat_id.0, lang_code).await.ok();

        let greeting = match lang_code {
            "en" => "🇬🇧 Language set to English.",
            "ru" => "🇷🇺 Язык установлен: русский.",
            "th" => "🇹🇭 ตั้งค่าภาษา: ไทย",
            "zh" => "🇨🇳 设置语言为中文。",
            _ => "Language set.",
        };

        bot.send_message(chat_id, greeting).await?;
    }

    Ok(())
}

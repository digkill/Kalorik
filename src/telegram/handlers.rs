use crate::db::queries;
use crate::locales::messages::Messages;
use crate::services::chart::draw_weekly_calories_chart;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile, Message, ParseMode},
};
use chrono::Utc;
use reqwest::Url;

pub async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let user_lang = queries::get_user(chat_id.0)
        .await
        .ok()
        .flatten()
        .and_then(|u| u.language_code)
        .unwrap_or("ru".to_string());
    let messages = Messages::get(&user_lang);

    if let Some(text) = msg.text() {
        if text == "/start" {
            queries::register_user(chat_id.0).await.ok();

            bot.send_message(chat_id, &messages.welcome).await?;

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
            return Ok(());
        }

        if text == "/help" {
            bot.send_message(chat_id, &messages.help_detailed)
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
            return Ok(());
        }

        if text == "/stats" {
            match queries::get_daily_summary(chat_id.0).await {
                Ok((calories, proteins, fats, carbs)) => {
                    let summary = format!(
                        "\u{1F4CA} *Daily Summary:*\nKcal: `{}`\nProtein: `{}`g\nFat: `{}`g\nCarbs: `{}`g",
                        calories, proteins, fats, carbs
                    );
                    bot.send_message(chat_id, summary)
                        .parse_mode(ParseMode::MarkdownV2)
                        .await?;
                }
                Err(e) => {
                    log::error!("Error in get_daily_summary: {}", e.to_string());
                    bot.send_message(chat_id, &messages.error).await?;
                }
            }
            return Ok(());
        }

        if text == "/reset" {
            match queries::reset_today_logs(chat_id.0).await {
                Ok(()) => {
                    bot.send_message(chat_id, &messages.reset_done).await?;
                }
                Err(e) => {
                    log::error!("Error in reset_today_logs: {}", e.to_string());
                    bot.send_message(chat_id, &messages.error).await?;
                }
            }
            return Ok(());
        }

        if text == "/week" {
            let weekly = queries::get_weekly_calories(chat_id.0)
                .await
                .unwrap_or_default();

            if weekly.is_empty() {
                bot.send_message(chat_id, &messages.week_empty).await?;
            } else {
                let data: Vec<(String, f32)> = weekly
                    .into_iter()
                    .map(|(date, val)| (date.format("%d.%m").to_string(), val))
                    .collect();

                let file_path = format!("temp/weekly_calories_{}.png", chat_id);
                if let Err(e) = std::fs::create_dir_all("temp") {
                    log::warn!("Failed to create temp directory: {}", e.to_string());
                }

                match draw_weekly_calories_chart(&data, &file_path) {
                    Ok(_) => {
                        if bot.send_photo(chat_id, InputFile::file(&file_path)).await.is_ok() {
                            if let Err(e) = std::fs::remove_file(&file_path) {
                                log::warn!("Failed to delete chart file {}: {}", file_path, e.to_string());
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Error drawing chart: {}", e.to_string());
                        bot.send_message(chat_id, &messages.graph_error).await?;
                    }
                }
            }
            return Ok(());
        }

        if text == "/subscribe" {
            handle_subscribe_command(&bot, &msg, &user_lang).await;
            return Ok(());
        }
        if text == "/status" {
            handle_status_command(&bot, &msg, &user_lang).await;
            return Ok(());
        }
        if text == "/cancel" {
            handle_cancel_command(&bot, &msg, &user_lang).await;
            return Ok(());
        }

        match crate::services::nutrition::analyze_food_description(text, &user_lang).await {
            Ok((summary, suggestion)) => {
                queries::add_food_log(
                    chat_id.0,
                    &summary.name,
                    summary.calories,
                    summary.proteins,
                    summary.fats,
                    summary.carbs,
                )
                    .await
                    .ok();

                let (cal, pr, fa, ch) = queries::get_daily_summary(chat_id.0)
                    .await
                    .unwrap_or_else(|e| {
                        log::warn!("get_daily_summary failed: {}", e.to_string());
                        (0.0, 0.0, 0.0, 0.0)
                    });
                let response = format!(
                    "✅ {}\n📊 Today: {:.0} kcal | 🥩 {:.1}P / 🧈 {:.1}F / 🍞 {:.1}C",
                    suggestion, cal, pr, fa, ch
                );
                bot.send_message(chat_id, response).await?;
            }
            Err(e) => {
                log::error!("Error in analyze_food_description: {}", e);
                bot.send_message(chat_id, &messages.unknown).await?;
            }
        }
        return Ok(());
    }

    if let Some(photos) = msg.photo() {
        if let Some(photo) = photos.last() {
            let file_id = &photo.file.id;
            let file = bot.get_file(file_id).send().await?;
            let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
            let url = format!("https://api.telegram.org/file/bot{}/{}", token, file.path);

            match crate::services::nutrition::analyze_image(&url, &user_lang).await {
                Ok((summary, suggestion)) => {
                    queries::add_food_log(
                        chat_id.0,
                        &summary.name,
                        summary.calories,
                        summary.proteins,
                        summary.fats,
                        summary.carbs,
                    )
                        .await
                        .ok();

                    let (cal, pr, fa, ch) = queries::get_daily_summary(chat_id.0)
                        .await
                        .unwrap_or_else(|e| {
                            log::warn!("get_daily_summary failed: {}", e.to_string());
                            (0.0, 0.0, 0.0, 0.0)
                        });
                    let response = format!(
                        "✅ {}\n📊 Today: {:.0} kcal | 🥩 {:.1}P / 🧈 {:.1}F / 🍞 {:.1}C",
                        suggestion, cal, pr, fa, ch
                    );
                    bot.send_message(chat_id, response).await?;
                }
                Err(e) => {
                    log::error!("Error in analyze_image: {}", e);
                    bot.send_message(chat_id, &messages.unknown).await?;
                }
            }
        }
        return Ok(());
    }

    if let Some(voice) = msg.voice() {
        let file_id = &voice.file.id;
        let file = bot.get_file(file_id).send().await?;
        let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
        let url = format!("https://api.telegram.org/file/bot{}/{}", token, file.path);

        match crate::services::nutrition::analyze_audio(&url, &user_lang).await {
            Ok((summary, suggestion)) => {
                queries::add_food_log(
                    chat_id.0,
                    &summary.name,
                    summary.calories,
                    summary.proteins,
                    summary.fats,
                    summary.carbs,
                )
                    .await
                    .ok();

                let (cal, pr, fa, ch) = queries::get_daily_summary(chat_id.0)
                    .await
                    .unwrap_or_else(|e| {
                        log::warn!("get_daily_summary failed: {}", e.to_string());
                        (0.0, 0.0, 0.0, 0.0)
                    });
                let response = format!(
                    "✅ {}\n📊 Today: {:.0} kcal | 🥩 {:.1}P / 🧈 {:.1}F / 🍞 {:.1}C",
                    suggestion, cal, pr, fa, ch
                );
                bot.send_message(chat_id, response).await?;
            }
            Err(e) => {
                log::error!("Error in analyze_audio: {}", e);
                bot.send_message(chat_id, &messages.unknown).await?;
            }
        }
        return Ok(());
    }

    bot.send_message(chat_id, &messages.unknown).await?;
    Ok(())
}

pub async fn handle_callback(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    if let Some(data) = q.data.as_deref() {
        let chat_id = q
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

pub async fn handle_subscribe(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let user_lang = queries::get_user(chat_id.0)
        .await
        .ok()
        .flatten()
        .and_then(|u| u.language_code)
        .unwrap_or("ru".to_string());
 //   let messages = Messages::get(&user_lang);

    let payment_url = format!("https://your-payment-provider.com/subscribe?user_id={}", chat_id.0);
    let subscribe_text = match user_lang.as_str() {
        "ru" => "🛒 Оформите подписку за 299 ₽ в месяц, чтобы продолжить пользоваться ботом!",
        "en" => "🛒 Subscribe for 299 RUB/month to continue using the bot!",
        "th" => "🛒 สมัครสมาชิกในราคา 299 รูเบิล/เดือน เพื่อใช้งานบอทต่อ!",
        "zh" => "🛒 每月299卢布订阅，以继续使用机器人！",
        _ => "🛒 Subscribe for 299 RUB/month to continue using the bot!",
    };

    bot.send_message(chat_id, subscribe_text)
        .reply_markup(InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::url("💳 Оформить подписку", Url::parse(&payment_url).unwrap())],
        ]))
        .await?;

    Ok(())
}

pub async fn check_subscription(chat_id: ChatId) -> bool {
    if let Ok(Some(user)) = queries::get_user(chat_id.0).await {
        if let Some(ends_at) = user.subscription_ends_at {
            return ends_at > Utc::now();
        }
    }
    false
}

pub async fn prompt_subscription(bot: &Bot, chat_id: ChatId, lang: &str) {
    let text = match lang {
        "ru" => "🔒 Доступно только по подписке. Подпишитесь за 299₽/мес для продолжения.",
        "en" => "🔒 Subscription required. Please subscribe for 299₽/month to continue.",
        "th" => "🔒 ต้องสมัครสมาชิก (299₽/เดือน) เพื่อใช้งานต่อ.",
        "zh" => "🔒 订阅需要。每月299₽继续使用。",
        _ => "🔒 Subscription required. Please subscribe.",
    };

    let payment_url = format!("https://your-payment-provider.com/subscribe?user_id={}", chat_id.0);
    let markup = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::url("💳 Оформить подписку", Url::parse(&payment_url).unwrap())],
    ]);

    bot.send_message(chat_id, text).reply_markup(markup).await.ok();
}

pub async fn send_daily_tip(bot: &Bot, chat_id: ChatId, lang: &str) {
    let tip = match lang {
        "ru" => "💡 Совет дня: Пей больше воды и следи за белками в рационе.",
        "en" => "💡 Tip: Drink more water and mind your protein intake.",
        "th" => "💡 เคล็ดลับ: ดื่มน้ำให้มากขึ้นและระวังโปรตีนในอาหาร.",
        "zh" => "💡 小贴士：多喝水，注意蛋白质摄入。",
        _ => "💡 Tip: Stay hydrated and eat balanced meals.",
    };
    bot.send_message(chat_id, tip).await.ok();
}

pub async fn handle_subscribe_command(bot: &Bot, msg: &Message, lang: &str) {
    let chat_id = msg.chat.id;
    let payment_url = format!("https://your-payment-provider.com/subscribe?user_id={}", chat_id.0);
    let subscribe_text = match lang {
        "ru" => "🛒 Оформите подписку за 299 ₽ в месяц, чтобы продолжить пользоваться ботом!",
        "en" => "🛒 Subscribe for 299 RUB/month to continue using the bot!",
        "th" => "🛒 สมัครสมาชิกในราคา 299 รูเบิล/เดือน เพื่อใช้งานบอทต่อ!",
        "zh" => "🛒 每月299卢布订阅，以继续使用机器人！",
        _ => "🛒 Subscribe for 299 RUB/month to continue using the bot!",
    };
    let markup = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::url("💳 Оформить подписку", Url::parse(&payment_url).unwrap())],
    ]);
    bot.send_message(chat_id, subscribe_text)
        .reply_markup(markup)
        .await
        .ok();
}

pub async fn handle_status_command(bot: &Bot, msg: &Message, lang: &str) {
    let chat_id = msg.chat.id;
    let active = check_subscription(chat_id).await;
    let text = match (active, lang) {
        (true, "ru") => "✅ Ваша подписка активна.",
        (true, _) => "✅ Your subscription is active.",
        (false, "ru") => "❌ Подписка не активна. Чтобы продолжить, оформите подписку.",
        (false, _) => "❌ Subscription inactive. Please subscribe to continue.",
    };
    bot.send_message(chat_id, text).await.ok();
}

pub async fn handle_cancel_command(bot: &Bot, msg: &Message, lang: &str) {
    let chat_id = msg.chat.id;
    let text = match lang {
        "ru" => "❗ Отменить подписку можно в разделе подписок вашего платёжного провайдера.",
        "en" => "❗ To cancel, go to your payment provider’s subscription section.",
        "th" => "❗ คุณสามารถยกเลิกได้ที่หน้าการสมัครสมาชิกของผู้ให้บริการชำระเงินของคุณ.",
        "zh" => "❗ 要取消，请转到付款提供商的订阅部分。",
        _ => "❗ To cancel, go to your payment provider’s subscription section.",
    };
    bot.send_message(chat_id, text).await.ok();
}
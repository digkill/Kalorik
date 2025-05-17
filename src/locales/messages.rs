use std::collections::HashMap;

#[derive(Clone)]
pub struct Messages {
    pub welcome: String,
    pub help: String,
    pub unknown: String,
}

impl Messages {
    pub fn get(lang: &str) -> Self {
        let mut langs = HashMap::new();

        langs.insert(
            "ru",
            Messages {
                welcome: "👋 Добро пожаловать! Отправь, что ты ел — и я всё посчитаю!".into(),
                help: "ℹ️ Отправь текст, фото еды или голосовое сообщение, и я дам информацию по калориям, БЖУ и рекомендациям.".into(),
                unknown: "⚠️ Пожалуйста, отправь текст, фото или голосовое сообщение.".into(),
            },
        );

        langs.insert(
            "en",
            Messages {
                welcome: "👋 Welcome! Tell me what you ate — and I'll calculate everything!".into(),
                help: "ℹ️ Send text, a food photo or a voice message, and I’ll provide calorie and nutrition info.".into(),
                unknown: "⚠️ Please send text, a photo or a voice message.".into(),
            },
        );

        langs.insert(
            "th",
            Messages {
                welcome: "👋 ยินดีต้อนรับ! บอกฉันว่าคุณกินอะไร — ฉันจะคำนวณให้หมดเลย!".into(),
                help: "ℹ️ ส่งข้อความ รูปอาหาร หรือเสียงมา แล้วฉันจะบอกข้อมูลโภชนาการให้คุณ.".into(),
                unknown: "⚠️ กรุณาส่งข้อความ รูป หรือเสียง.".into(),
            },
        );

        langs.insert(
            "zh",
            Messages {
                welcome: "👋 欢迎！告诉我你吃了什么，我来帮你计算！".into(),
                help: "ℹ️ 请发送文本、食物照片或语音，我将提供热量和营养信息。".into(),
                unknown: "⚠️ 请发送文字、图片或语音信息。".into(),
            },
        );

        langs.get(lang).cloned().unwrap_or_else(|| langs["en"].clone())
    }
}

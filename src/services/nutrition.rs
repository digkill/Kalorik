use reqwest::Client;
use std::env;

pub async fn analyze_food_description(text: &str) -> String {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let body = serde_json::json!({
        "model": "gpt-4o",
        "messages": [{ "role": "user", "content": format!("Сколько калорий и БЖУ в: {}", text) }]
    });
    let client = Client::new();
    let res = client.post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send().await.unwrap();
    res.json::<serde_json::Value>().await.unwrap()["choices"][0]["message"]["content"].as_str().unwrap().to_string()
}

pub async fn analyze_image(url: &str) -> String {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let body = serde_json::json!({
        "model": "gpt-4o",
        "messages": [{
            "role": "user",
            "content": [
                { "type": "text", "text": "Что изображено на этой еде? Сколько калорий и БЖУ?" },
                { "type": "image_url", "image_url": { "url": url } }
            ]
        }]
    });
    let client = Client::new();
    let res = client.post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send().await.unwrap();
    res.json::<serde_json::Value>().await.unwrap()["choices"][0]["message"]["content"].as_str().unwrap().to_string()
}

pub async fn analyze_audio(url: &str) -> String {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let file_bytes = reqwest::get(url).await.unwrap().bytes().await.unwrap();
    let part = reqwest::multipart::Part::stream(file_bytes.clone())
        .file_name("audio.ogg")
        .mime_str("audio/ogg").unwrap();
    let form = reqwest::multipart::Form::new().part("file", part).text("model", "whisper-1");

    let client = Client::new();
    let res = client.post("https://api.openai.com/v1/audio/transcriptions")
        .bearer_auth(api_key)
        .multipart(form)
        .send().await.unwrap();

    let text = res.json::<serde_json::Value>().await.unwrap()["text"].as_str().unwrap().to_string();
    analyze_food_description(&text).await
}

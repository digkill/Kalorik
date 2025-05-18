use reqwest::Client;
use std::env;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NutritionError(String);

impl fmt::Display for NutritionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for NutritionError {}

impl From<reqwest::Error> for NutritionError {
    fn from(err: reqwest::Error) -> Self {
        NutritionError(err.to_string())
    }
}

impl From<serde_json::Error> for NutritionError {
    fn from(err: serde_json::Error) -> Self {
        NutritionError(err.to_string())
    }
}

impl From<std::env::VarError> for NutritionError {
    fn from(err: std::env::VarError) -> Self {
        NutritionError(err.to_string())
    }
}

impl From<&str> for NutritionError {
    fn from(err: &str) -> Self {
        NutritionError(err.to_string())
    }
}

impl From<std::num::ParseFloatError> for NutritionError {
    fn from(err: std::num::ParseFloatError) -> Self {
        NutritionError(err.to_string())
    }
}

impl From<regex::Error> for NutritionError {
    fn from(err: regex::Error) -> Self {
        NutritionError(err.to_string())
    }
}

#[derive(Debug)]
pub struct FoodSummary {
    pub name: String,
    pub calories: Option<f32>,
    pub proteins: Option<f32>,
    pub fats: Option<f32>,
    pub carbs: Option<f32>,
}

pub async fn analyze_food_description(text: &str) -> Result<(FoodSummary, String), NutritionError> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let body = serde_json::json!({
        "model": "gpt-4o",
        "messages": [{
            "role": "user",
            "content": format!("Рассчитай калории и БЖУ для: {}", text)
        }],
        "temperature": 0.3
    });

    let client = Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("No content")?
        .to_string();

    let name = text.to_string();
    let calories = extract_float(&content, "ккал").or_else(|| extract_float(&content, "kcal"));
    let proteins = extract_float(&content, "белк").or_else(|| extract_float(&content, "protein"));
    let fats = extract_float(&content, "жир");
    let carbs = extract_float(&content, "углев");

    Ok((
        FoodSummary {
            name,
            calories,
            proteins,
            fats,
            carbs,
        },
        content,
    ))
}

fn extract_float(text: &str, key: &str) -> Option<f32> {
    use regex::Regex;
    let re = Regex::new(&format!(r"(?i)([\d.,]+)\s*{}", key)).ok()?;
    let cap = re.captures(text)?;
    let val_str = cap.get(1)?.as_str().replace(",", ".");
    val_str.parse::<f32>().ok()
}

pub async fn analyze_image(url: &str) -> Result<(FoodSummary, String), NutritionError> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let body = serde_json::json!({
        "model": "gpt-4o",
        "messages": [{
            "role": "user",
            "content": [
                { "type": "text", "text": "Что изображено на этой еде? Сколько калорий и БЖУ?" },
                { "type": "image_url", "image_url": { "url": url } }
            ]
        }],
        "temperature": 0.3
    });

    let client = Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?;

    let json = res.json::<serde_json::Value>().await?;
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("No content")?
        .to_string();

    let name = "Еда с фото".to_string();
    let calories = extract_float(&content, "ккал").or_else(|| extract_float(&content, "kcal"));
    let proteins = extract_float(&content, "белк").or_else(|| extract_float(&content, "protein"));
    let fats = extract_float(&content, "жир");
    let carbs = extract_float(&content, "углев");

    Ok((
        FoodSummary {
            name,
            calories,
            proteins,
            fats,
            carbs,
        },
        content,
    ))
}

pub async fn analyze_audio(url: &str) -> Result<(FoodSummary, String), NutritionError> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let file_bytes = reqwest::get(url).await?.bytes().await?;
    let part = reqwest::multipart::Part::stream(file_bytes)
        .file_name("audio.ogg")
        .mime_str("audio/ogg")?;
    let form = reqwest::multipart::Form::new()
        .part("file", part)
        .text("model", "whisper-1");

    let client = Client::new();
    let res = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .bearer_auth(api_key)
        .multipart(form)
        .send()
        .await?;

    let json = res.json::<serde_json::Value>().await?;
    let text = json["text"]
        .as_str()
        .ok_or("No transcription text")?
        .to_string();

    analyze_food_description(&text).await
}
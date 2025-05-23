use reqwest::Client;
use std::env;
use std::error::Error;
use std::fmt;

const LANG_PROMPTS: &[(&str, &str, &str, &str)] = &[
    ("ru", "Отвечай на русском языке.", "Ответ должен быть строго в формате JSON: {\"name\": \"...\", \"calories\": ..., \"proteins\": ..., \"fats\": ..., \"carbs\": ...}", "Рассчитай калории и БЖУ для"),
    ("en", "Answer in English.", "The answer must be strictly in JSON format: {\"name\": \"...\", \"calories\": ..., \"proteins\": ..., \"fats\": ..., \"carbs\": ...}", "Calculate calories and Proteins Fats Carbohydrates for"),
    ("th", "ตอบเป็นภาษาไทย.", "คำตอบต้องอยู่ในรูปแบบ JSON เท่านั้น: {\"name\": \"...\", \"calories\": ..., \"proteins\": ..., \"fats\": ..., \"carbs\": ...}", "คำนวณแคลอรี่และโปรตีน ไขมัน คาร์โบไฮเดรตสำหรับ"),
    ("zh", "请用中文回答。", "回答必须严格采用 JSON 格式: {\"name\": \"...\", \"calories\": ..., \"proteins\": ..., \"fats\": ..., \"carbs\": ...}", "计算卡路里和蛋白质脂肪碳水化合物"),
];

fn get_lang_prompt(lang: &str) -> (&str, &str, &str) {
    LANG_PROMPTS
        .iter()
        .find(|&&(code, _, _, _)| code == lang)
        .map(|&(_, p1, p2, p3)| (p1, p2, p3))
        .unwrap_or((
            "Answer in English.",
            "The answer must be strictly in JSON format: {\"name\": \"...\", \"calories\": ..., \"proteins\": ..., \"fats\": ..., \"carbs\": ...}",
            "Calculate calories and Proteins Fats Carbohydrates for"
        ))
}


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

pub async fn analyze_food_description(text: &str, lang: &str) -> Result<(FoodSummary, String), NutritionError> {
    let (lang_prompt, format_prompt, prompt) = get_lang_prompt(lang);

    let api_key = env::var("OPENAI_API_KEY")?;
    let body = serde_json::json!({
        "model": "gpt-4o",
        "messages": [
            {
                "role": "system",
                "content": format!("{} {}", lang_prompt, format_prompt)
            },
            {
                "role": "user",
                "content": format!("{}: {}", prompt, text)
            }
        ],
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
    let fats = extract_float(&content, "жир").or_else(|| extract_float(&content, "fats"));
    let carbs = extract_float(&content, "углев").or_else(|| extract_float(&content, "carbs"));

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
    let re = Regex::new(&format!(r"(?i)([\\d.,]+)\\s*{}", key)).ok()?;
    let cap = re.captures(text)?;
    let val_str = cap.get(1)?.as_str().replace(",", ".");
    val_str.parse::<f32>().ok()
}

pub async fn analyze_image(url: &str, lang: &str) -> Result<(FoodSummary, String), NutritionError> {
    let (lang_prompt, _, _) = get_lang_prompt(lang);

    let api_key = env::var("OPENAI_API_KEY")?;
    let body = serde_json::json!({
        "model": "gpt-4o",
        "messages": [{
            "role": "user",
            "content": [
                { "type": "text", "text": format!("{} What is depicted on this food? How many calories and Proteins Fats Carbohydrates?", lang_prompt) },
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

pub async fn analyze_audio(url: &str, lang: &str) -> Result<(FoodSummary, String), NutritionError> {
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

    analyze_food_description(&text, lang).await
}

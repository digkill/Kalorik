use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub chat_id: i64,
    pub username: Option<String>,
    pub age: Option<i32>,
    pub weight_kg: Option<f64>,
    pub height_cm: Option<f64>,
    pub imt: Option<f64>,
    pub gender: Option<String>,
    pub activity_level: Option<String>,
    pub goal: Option<String>,
    pub language_code: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub subscription_ends_at: Option<DateTime<Utc>>, // <-- добавлено поле подписки
}

impl User {
    pub fn is_subscription_active(&self) -> bool {
        match self.subscription_ends_at {
            Some(ends_at) => ends_at > Utc::now(),
            None => false,
        }
    }

    pub fn extend_subscription(&mut self, months: i64) {
        let now = Utc::now();
        let base = self.subscription_ends_at.unwrap_or(now);
        self.subscription_ends_at = Some(base + Duration::days(30 * months));
    }
}

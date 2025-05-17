use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDateTime, Utc};

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
    pub created_at: Option<NaiveDateTime>,
}

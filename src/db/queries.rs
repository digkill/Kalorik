use crate::db::models::User;
use sqlx::PgPool;
use std::sync::OnceLock;
use chrono::{NaiveDate, Utc, DateTime};

pub static DB_POOL: OnceLock<PgPool> = OnceLock::new();

/// Устанавливает глобальный пул соединений с БД.
pub fn set_pool(pool: PgPool) {
    if DB_POOL.set(pool).is_err() {
        eprintln!("⚠ DB_POOL уже был установлен.");
    }
}

/// Регистрирует пользователя, если он ещё не существует.
pub async fn register_user(chat_id: i64) -> Result<(), sqlx::Error> {
    let Some(pool) = DB_POOL.get() else {
        return Err(sqlx::Error::PoolTimedOut);
    };

    sqlx::query!(
        r#"
        INSERT INTO users (chat_id, created_at, updated_at)
        VALUES ($1, $2, $3)
        ON CONFLICT (chat_id) DO NOTHING
        "#,
        chat_id,
        Utc::now(),
        Utc::now()
    )
        .execute(pool)
        .await
        .map_err(|e| {
            log::warn!("Failed to register user {}: {}", chat_id, e);
            e
        })?;

    Ok(())
}

/// Обновляет язык пользователя.
pub async fn update_language(chat_id: i64, lang: &str) -> Result<(), sqlx::Error> {
    let Some(pool) = DB_POOL.get() else {
        return Err(sqlx::Error::PoolTimedOut);
    };

    sqlx::query!(
        "UPDATE users SET language_code = $1 WHERE chat_id = $2",
        lang,
        chat_id
    )
        .execute(pool)
        .await
        .map_err(|e| {
            log::warn!("Failed to update language for {}: {}", chat_id, e);
            e
        })?;

    Ok(())
}

/// Получает данные пользователя по chat_id.
pub async fn get_user(chat_id: i64) -> Result<Option<User>, sqlx::Error> {
    let Some(pool) = DB_POOL.get() else {
        return Err(sqlx::Error::PoolTimedOut);
    };

    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, chat_id, username, age, weight_kg, height_cm, gender,
               activity_level, goal, imt, created_at, language_code, updated_at
        FROM users
        WHERE chat_id = $1
        "#,
        chat_id
    )
        .fetch_optional(pool)
        .await?;

    if user.is_none() {
        log::info!("User not found: {}", chat_id);
    }

    Ok(user)
}

/// Получает суммарные калории и БЖУ за текущие сутки.
pub async fn get_daily_summary(chat_id: i64) -> Result<(f32, f32, f32, f32), sqlx::Error> {
    let Some(pool) = DB_POOL.get() else {
        return Err(sqlx::Error::PoolTimedOut);
    };

    let today = Utc::now().date_naive();
    let start: DateTime<Utc> = DateTime::from_naive_utc_and_offset(
        today.and_hms_opt(0, 0, 0).unwrap(),
        Utc,
    );
    let end: DateTime<Utc> = DateTime::from_naive_utc_and_offset(
        today.and_hms_opt(23, 59, 59).unwrap(),
        Utc,
    );

    let result = sqlx::query!(
        r#"
        SELECT
            COALESCE(SUM(calories), 0) as calories,
            COALESCE(SUM(proteins), 0) as proteins,
            COALESCE(SUM(fats), 0) as fats,
            COALESCE(SUM(carbs), 0) as carbs
        FROM food_logs
        WHERE chat_id = $1
        AND created_at BETWEEN $2 AND $3
        "#,
        chat_id,
        start,
        end
    )
        .fetch_one(pool)
        .await?;

    Ok((
        result.calories.unwrap_or(0.0),
        result.proteins.unwrap_or(0.0),
        result.fats.unwrap_or(0.0),
        result.carbs.unwrap_or(0.0),
    ))
}

/// Добавляет запись о приёме пищи в лог.
pub async fn add_food_log(
    chat_id: i64,
    food_name: &str,
    calories: Option<f32>,
    proteins: Option<f32>,
    fats: Option<f32>,
    carbs: Option<f32>,
) -> Result<(), sqlx::Error> {
    let Some(pool) = DB_POOL.get() else {
        return Err(sqlx::Error::PoolTimedOut);
    };

    sqlx::query!(
        r#"
        INSERT INTO food_logs (chat_id, food_name, calories, proteins, fats, carbs, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        chat_id,
        food_name,
        calories,
        proteins,
        fats,
        carbs,
        Utc::now()
    )
        .execute(pool)
        .await
        .map_err(|e| {
            log::warn!("Failed to insert food log for {}: {}", chat_id, e);
            e
        })?;

    Ok(())
}

/// Получает суммарные калории по дням за последние 7 суток.
pub async fn get_weekly_calories(chat_id: i64) -> Result<Vec<(NaiveDate, f32)>, sqlx::Error> {
    let Some(pool) = DB_POOL.get() else {
        return Err(sqlx::Error::PoolTimedOut);
    };

    let result = sqlx::query!(
        r#"
        SELECT
            DATE(created_at) as date,
            SUM(calories) as total_calories
        FROM food_logs
        WHERE chat_id = $1 AND created_at > now() - interval '7 days'
        GROUP BY date
        ORDER BY date
        "#,
        chat_id
    )
        .fetch_all(pool)
        .await?;

    Ok(result
        .into_iter()
        .map(|r| (r.date.unwrap(), r.total_calories.unwrap_or(0.0)))
        .collect())
}

/// Удаляет записи за текущие сутки.
pub async fn reset_today_logs(chat_id: i64) -> Result<(), sqlx::Error> {
    let Some(pool) = DB_POOL.get() else {
        return Err(sqlx::Error::PoolTimedOut);
    };

    sqlx::query!(
        "DELETE FROM food_logs WHERE chat_id = $1 AND created_at::date = CURRENT_DATE",
        chat_id
    )
        .execute(pool)
        .await?;

    Ok(())
}
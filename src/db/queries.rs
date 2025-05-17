use crate::db::models::User;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::OnceLock;

pub static DB_POOL: OnceLock<PgPool> = OnceLock::new();

/// Устанавливает глобальный пул соединений с БД.
pub fn set_pool(pool: PgPool) {
    if DB_POOL.set(pool).is_err() {
        eprintln!("⚠DB_POOL уже был установлен.");
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
        .await?;

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
        .await?;

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

    Ok(user)
}

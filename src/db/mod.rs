pub mod queries;
pub mod models;

use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn init() -> Result<PgPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPoolOptions::new().connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
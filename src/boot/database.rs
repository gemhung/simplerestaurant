use crate::config::settings;
use std::str::FromStr;

pub static DB: std::sync::OnceLock<sqlx::Pool<sqlx::Postgres>> = std::sync::OnceLock::new();

#[inline]
pub fn db() -> &'static sqlx::Pool<sqlx::Postgres> {
    DB.get().unwrap()
}

#[must_use]
pub async fn init() -> Result<(), sqlx::Error> {
    let pool = load().await?;
    DB.set(pool).unwrap();

    Ok(())
}

#[must_use]
pub async fn load() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let settings = settings();
    let connect_opts = sqlx::postgres::PgConnectOptions::from_str(&settings.database_url).unwrap();
    let parallel_num = std::thread::available_parallelism()
        .map(|inner| inner.get())
        .unwrap_or(10);
    sqlx::postgres::PgPoolOptions::new()
        .max_connections((parallel_num * 2) as u32)
        .min_connections(parallel_num as u32)
        .connect_with(connect_opts)
        .await
}


use crate::config::configuration::DatabaseSettings;
use crate::config::settings;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::str::FromStr;

pub async fn load() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let settings = crate::config::configuration::settings();
    let connect_opts = settings.database.connect_options();
    let parallel_num = std::thread::available_parallelism()
        .map(|inner| inner.get())
        .unwrap_or(10);
    sqlx::postgres::PgPoolOptions::new()
        .max_connections((parallel_num * 2) as u32)
        .min_connections(parallel_num as u32)
        .connect_with(connect_opts)
        .await
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.connect_options())
}

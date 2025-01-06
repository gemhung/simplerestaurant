use crate::config::configuration::DatabaseSettings;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .test_before_acquire(false)
        .connect_lazy_with(configuration.connect_options())
}

pub async fn get_connection_pool2(configuration: &DatabaseSettings) -> PgPool {
    let connect_opts = configuration.connect_options();
    let parallel_num = std::thread::available_parallelism()
        .map(|inner| inner.get())
        .unwrap_or(10);
    sqlx::postgres::PgPoolOptions::new()
        .max_connections((parallel_num * 2) as u32)
        .min_connections(parallel_num as u32)
        .connect_with(connect_opts)
        .await
        .unwrap()
}

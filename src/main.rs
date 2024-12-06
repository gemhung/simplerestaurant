#![allow(unused)]
mod boot;
mod config;
mod controllers;
mod errors;
mod idempotency;
mod models;
mod utils;

use crate::config::settings;

fn main() -> Result<(), crate::errors::AppError> {
    boot::telemetry::load();
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            {
                let _config = config::configuration::get_configuration()
                    .expect("Failed to load configuration");
                tracing::debug!("{_config:?}");
                let _ = boot::database::init().await;
            }
            // Launching web server
            boot::app::launch().await
        })?;

    Ok(())
}

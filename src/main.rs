use simplerestaurant::boot;
use simplerestaurant::config;
use simplerestaurant::errors;

fn main() -> Result<(), crate::errors::AppError> {
    boot::telemetry::load();
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let config =
                config::configuration::get_configuration().expect("Failed to load configuration");
            tracing::debug!("{config:?}");
            // Launching web server
            let (server, _port) = boot::app::launch(config).await?;
            server.await?;

            Result::<(), crate::errors::AppError>::Ok(())
        })?;

    Ok(())
}

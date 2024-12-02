mod config;
mod boot;
mod errors;

use config::settings;

fn main() -> Result<(), crate::errors::AppError> {
    boot::telemetry::load();
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            {
                let _ = config::settings::init_settings();
                //let _ = aws::init_aws().await;
                let _ = boot::database::init().await;
                //let _ = boot::redis::init().unwrap();
            }
            // Launching web server
            //config::boot::launch().await
            todo!()
        });

    Ok(())
}



use secrecy::Secret;
use simplerestaurant::boot::telemetry::get_subscriber;
use simplerestaurant::boot::telemetry::init_subscriber;
use simplerestaurant::config::configuration::get_configuration;
use simplerestaurant::config::configuration::DatabaseSettings;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::sync::LazyLock;
use uuid::Uuid;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub api_client: reqwest::Client,
}

impl TestApp {
    pub async fn post_create_orders<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(format!("{}/items", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_all_items(&self, table: i32) -> reqwest::Response {
        self.api_client
            .get(format!("{}/items", &self.address))
            .query(&[("table", table.to_string())])
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_specified_items(&self, table: i32, name: &str) -> reqwest::Response {
        self.api_client
            .get(format!("{}/items/{}", &self.address, name))
            .query(&[("table", table.to_string())])
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn delete_orders<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .delete(format!("{}/items", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);

    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application.port = 0;
        c
    };

    // Create and migrate the database
    configure_database(&configuration.database).await;

    // Launch the application as a background task
    let (server, application_port) = simplerestaurant::boot::app::launch(configuration)
        .await
        .expect("Failed to launch");
    let _ = tokio::spawn(server);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    TestApp {
        address: format!("http://localhost:{}", application_port),
        //db_pool: database::load().await.expect("Database failed to load"),
        api_client: client,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let maintenance_settings = DatabaseSettings {
        database_name: "postgres".to_string(),
        username: "postgres".to_string(),
        password: Secret::new("password".to_string()),
        ..config.clone()
    };
    let mut connection = PgConnection::connect_with(&maintenance_settings.connect_options())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.connect_options())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

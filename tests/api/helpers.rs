use secrecy::Secret;
use simplerestaurant::boot::database;
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
    pub port: u16,
    //pub db_pool: PgPool,
    //pub email_server: MockServer,
    //pub test_user: TestUser,
    pub api_client: reqwest::Client,
    //pub email_client: EmailClient,
}

/// Confirmation links embedded in the request to the email API.
pub struct ConfirmationLinks {
    pub html: reqwest::Url,
    pub plain_text: reqwest::Url,
}

impl TestApp {
    //pub async fn dispatch_all_pending_emails(&self) {
    //    loop {
    //        if let ExecutionOutcome::EmptyQueue =
    //            try_execute_task(&self.db_pool, &self.email_client)
    //                .await
    //                .unwrap()
    //        {
    //            break;
    //        }
    //    }
    //}

    //pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
    //    self.api_client
    //        .post(&format!("{}/subscriptions", &self.address))
    //        .header("Content-Type", "application/x-www-form-urlencoded")
    //        .body(body)
    //        .send()
    //        .await
    //        .expect("Failed to execute request.")
    //}

    //pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    //where
    //    Body: serde::Serialize,
    //{
    //    self.api_client
    //        .post(&format!("{}/login", &self.address))
    //        .form(body)
    //        .send()
    //        .await
    //        .expect("Failed to execute request.")
    //}

    //pub async fn get_login_html(&self) -> String {
    //    self.api_client
    //        .get(&format!("{}/login", &self.address))
    //        .send()
    //        .await
    //        .expect("Failed to execute request.")
    //        .text()
    //        .await
    //        .unwrap()
    //}

    //pub async fn get_admin_dashboard(&self) -> reqwest::Response {
    //    self.api_client
    //        .get(&format!("{}/admin/dashboard", &self.address))
    //        .send()
    //        .await
    //        .expect("Failed to execute request.")
    //}

    //pub async fn get_admin_dashboard_html(&self) -> String {
    //    self.get_admin_dashboard().await.text().await.unwrap()
    //}

    //pub async fn get_change_password(&self) -> reqwest::Response {
    //    self.api_client
    //        .get(&format!("{}/admin/password", &self.address))
    //        .send()
    //        .await
    //        .expect("Failed to execute request.")
    //}

    //pub async fn get_change_password_html(&self) -> String {
    //    self.get_change_password().await.text().await.unwrap()
    //}

    //pub async fn post_logout(&self) -> reqwest::Response {
    //    self.api_client
    //        .post(&format!("{}/admin/logout", &self.address))
    //        .send()
    //        .await
    //        .expect("Failed to execute request.")
    //}

    //pub async fn post_change_password<Body>(&self, body: &Body) -> reqwest::Response
    //where
    //    Body: serde::Serialize,
    //{
    //    self.api_client
    //        .post(&format!("{}/admin/password", &self.address))
    //        .form(body)
    //        .send()
    //        .await
    //        .expect("Failed to execute request.")
    //}

    //pub async fn get_publish_newsletter(&self) -> reqwest::Response {
    //    self.api_client
    //        .get(&format!("{}/admin/newsletters", &self.address))
    //        .send()
    //        .await
    //        .expect("Failed to execute request.")
    //}

    //pub async fn get_publish_newsletter_html(&self) -> String {
    //    self.get_publish_newsletter().await.text().await.unwrap()
    //}

    pub async fn post_create_orders<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/items", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_all_items(&self, table: i32) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/items", &self.address))
            .query(&[("table", table.to_string())])
            .send()
            .await
            .expect("Failed to execute request.")
    }

    ///// Extract the confirmation links embedded in the request to the email API.
    //pub fn get_confirmation_links(&self, email_request: &wiremock::Request) -> ConfirmationLinks {
    //    let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

    //    // Extract the link from one of the request fields.
    //    let get_link = |s: &str| {
    //        let links: Vec<_> = linkify::LinkFinder::new()
    //            .links(s)
    //            .filter(|l| *l.kind() == linkify::LinkKind::Url)
    //            .collect();
    //        assert_eq!(links.len(), 1);
    //        let raw_link = links[0].as_str().to_owned();
    //        let mut confirmation_link = reqwest::Url::parse(&raw_link).unwrap();
    //        // Let's make sure we don't call random APIs on the web
    //        assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");
    //        confirmation_link.set_port(Some(self.port)).unwrap();
    //        confirmation_link
    //    };

    //    let html = get_link(body["HtmlBody"].as_str().unwrap());
    //    let plain_text = get_link(body["TextBody"].as_str().unwrap());
    //    ConfirmationLinks { html, plain_text }
    //}
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

    let test_app = TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
        //db_pool: database::load().await.expect("Database failed to load"),
        api_client: client,
    };

    test_app
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

pub fn assert_is_redirect_to(response: &reqwest::Response, location: &str) {
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), location);
}

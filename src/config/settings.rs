use serde::Deserialize;
use std::path::Path;
use std::sync::OnceLock;

pub fn init_settings() {
    let _  = settings();
}

pub fn settings() -> &'static Settings {
    static SETTINGS: OnceLock<Settings> = OnceLock::new();
    SETTINGS.get_or_init(Settings::new)
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Settings {
    pub database_url: String,
}

impl Settings {
    fn new() -> Settings {
        dotenvy::dotenv_override().unwrap();
        let vars =
            serde_json::to_value(std::env::vars().collect::<std::collections::HashMap<_, _>>())
                .unwrap();

        serde_json::from_value(vars).unwrap()
    }
}

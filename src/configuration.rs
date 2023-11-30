use std::env;

use anyhow::Result;
use config::{Config, Environment};
use serde::Deserialize;
use tracing::debug;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub app: AppSettings,
    pub log_dir: String,
}

#[derive(Deserialize, Debug)]
pub struct AppSettings {
    pub host: String,
    pub port: u16,
}

pub enum AppEnv {
    Development,
    Production,
}

impl TryFrom<String> for AppEnv {
    type Error = String;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "production" => Ok(Self::Production),
            _ => Ok(Self::Development),
        }
    }
}

impl AppEnv {
    fn as_str(self) -> String {
        match self {
            Self::Development => "development".into(),
            Self::Production => "production".into(),
        }
    }
}

pub fn get_configuration() -> Result<Settings> {
    let mut settings = Config::builder();
    // Add in `./Settings.toml`
    // .add_source(config::File::with_name("configuration"))

    let current_dir = env::current_dir()?;
    let configuration_dir = current_dir.join("configuration");

    let env: AppEnv = env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_err| "development".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    settings = settings
        .add_source(config::File::with_name(
            configuration_dir
                .join("base")
                .to_str()
                .expect("base configuration not existed"),
        ))
        .add_source(config::File::with_name(
            configuration_dir
                .join(env.as_str())
                .to_str()
                .expect("env configuration not existed"),
        ))
        .add_source(
            Environment::with_prefix("config")
                .try_parsing(true)
                .separator("_"),
        );

    let settings = settings.build()?.try_deserialize()?;

    debug!("{:?}", settings);

    Ok(settings)
}

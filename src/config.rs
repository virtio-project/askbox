use serde::Deserialize;
use sqlx::postgres::PgConnectOptions;
use std::{env, fs};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub host: Host,
    pub database: Database,
    pub hcaptcha: HCaptcha,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Host {
    #[serde(default = "default_bind")]
    pub bind: String,
    pub hostname: String,
}


#[derive(Clone, Debug, Deserialize)]
pub struct Database {
    pub username: String,
    pub password: String,
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub database: String,
}

fn default_bind() -> String {
    "127.0.0.1:8080".to_string()
}
const fn default_port() -> u16 {
    5432
}

#[derive(Clone, Debug, Deserialize)]
pub struct HCaptcha {
    #[serde(rename = "site-key")]
    pub site_key: String,
    pub secret: String,
}

impl From<&Database> for PgConnectOptions {
    fn from(d: &Database) -> Self {
        PgConnectOptions::default()
            .username(d.username.as_str())
            .password(d.password.as_str())
            .host(d.host.as_str())
            .port(d.port)
            .database(d.database.as_str())
    }
}

impl Default for Config {
    fn default() -> Self {
        let path = env::var("ASKBOX_CONFIG").unwrap_or_else(|_| "config.toml".to_string());
        fs::read(path)
            .ok()
            .and_then(|s| toml::from_slice(&s).ok())
            .unwrap()
    }
}
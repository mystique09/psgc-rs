use std::env;

use crate::config::ConfigError;

#[derive(Debug, bon::Builder)]
pub struct DatabaseConfig {
    pub db_host: String,
    pub db_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_name: String,
    pub db_url: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().unwrap();

        let db_host = env::var("DATABASE_HOST")
            .map_err(|_| ConfigError::MissingEnv("missing DATABASE_HOST".to_string()))?;
        let db_port = env::var("DATABASE_PORT")
            .map_err(|_| ConfigError::MissingEnv("missing DATABASE_PORT".to_string()))?
            .parse::<u16>()
            .map_err(|_| ConfigError::InvalidValue("DATABASE_PORT".to_string()))?;
        let db_username = env::var("DATABASE_USERNAME")
            .map_err(|_| ConfigError::MissingEnv("missing DATABASE_USERNAME".to_string()))?;
        let db_password = env::var("DATABASE_PASSWORD")
            .map_err(|_| ConfigError::MissingEnv("missing DATABASE_PASSWORD".to_string()))?;
        let db_name = env::var("DATABASE_NAME")
            .map_err(|_| ConfigError::MissingEnv("missing DATABASE_NAME".to_string()))?;

        let db_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingEnv("missing DATABASE_URL".to_string()))
            .unwrap_or("postgresql://postgres:secret@127.0.0.1:5432/psgc-rs".to_string());

        Ok(Self::builder()
            .db_host(db_host)
            .db_port(db_port)
            .db_username(db_username)
            .db_password(db_password)
            .db_name(db_name)
            .db_url(db_url)
            .build())
    }
}

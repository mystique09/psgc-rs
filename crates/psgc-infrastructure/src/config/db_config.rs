use std::env;

use crate::config::ConfigError;

#[derive(Debug, bon::Builder)]
pub struct DatabaseConfig {
    pub db_url: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().unwrap();

        let db_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingEnv("missing DATABASE_URL".to_string()))
            .unwrap_or(
                "postgresql://postgres:secret@localhost:5432/psgc-rs?sslmode=disable".to_string(),
            );

        Ok(Self::builder().db_url(db_url).build())
    }
}

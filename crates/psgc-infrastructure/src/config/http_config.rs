use std::{env, net::Ipv4Addr};

use crate::config::ConfigError;

#[derive(Debug, Clone, bon::Builder)]
pub struct HTTPConfig {
    pub host: Ipv4Addr,
    pub port: u16,
    pub allowed_origins: Vec<String>,
}

impl HTTPConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().unwrap();

        let host = std::env::var("HOST")
            .map_err(|_| ConfigError::MissingEnv("HOST".to_string()))?
            .parse::<Ipv4Addr>()
            .map_err(|_| ConfigError::InvalidValue("HOST".to_string()))?;

        let port = std::env::var("PORT")
            .map_err(|_| ConfigError::MissingEnv("PORT".to_string()))?
            .parse::<u16>()
            .map_err(|_| ConfigError::InvalidValue("PORT".to_string()))?;

        let allowed_origins = env::var("ALLOWED_ORIGINS")
            .map_err(|_| ConfigError::MissingEnv("missing ALLOWED_ORIGINS".to_string()))
            .unwrap_or(
                "postgresql://postgres:secret@localhost:5432/psgc-rs?sslmode=disable".to_string(),
            )
            .split(",")
            .map(|o| o.to_string())
            .collect::<Vec<String>>();

        Ok(Self::builder()
            .host(host)
            .port(port)
            .allowed_origins(allowed_origins)
            .build())
    }

    pub fn get_connection_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

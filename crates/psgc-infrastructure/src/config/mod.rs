pub mod db_config;
pub mod http_config;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing env {0}")]
    MissingEnv(String),
    #[error("invalid parsed value {0}")]
    InvalidValue(String),
}

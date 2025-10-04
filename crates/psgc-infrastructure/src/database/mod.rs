pub(crate) mod mappers;
pub(crate) mod models;
pub mod seeder;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum DatabaseSeedError {
    #[error("{0}")]
    DbError(rbatis::Error),
    #[error("{0}")]
    Serialization(serde_json::Error),
    #[error("{0}")]
    Connection(String),
    #[error("{0}")]
    Internal(String),
}

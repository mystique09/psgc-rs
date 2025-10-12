use thiserror::Error;

#[derive(Debug, Error)]
pub enum UsecaseError {
    #[error("Entity not found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
    #[error("Unauthorized access")]
    Unauthorized,
    #[error("Forbidden operation")]
    Forbidden,
    #[error("Conflict: {0}")]
    Conflict(String),
}

impl From<psgc_domain::errors::RepositoryError> for UsecaseError {
    fn from(error: psgc_domain::errors::RepositoryError) -> Self {
        match error {
            psgc_domain::errors::RepositoryError::NotFound => UsecaseError::NotFound,
            psgc_domain::errors::RepositoryError::ConstraintViolation(msg) => {
                UsecaseError::ConstraintViolation(msg)
            }
            psgc_domain::errors::RepositoryError::DatabaseError(msg) => {
                UsecaseError::DatabaseError(msg)
            }
            psgc_domain::errors::RepositoryError::UnexpectedError(msg) => {
                UsecaseError::UnexpectedError(msg)
            }
        }
    }
}

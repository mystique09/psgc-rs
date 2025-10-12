use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(tag = "type")]
pub enum APIError {
    #[schema(example = "Entity not found")]
    NotFound,
    #[schema(example = "Validation failed: Invalid input")]
    ValidationError(String),
    #[schema(example = "Database connection failed")]
    InternalError(String),
    #[schema(example = "Forbidden operation")]
    Forbidden,
    #[schema(example = "Unauthorized access")]
    Unauthorized,
    #[schema(example = "Resource already exists")]
    Conflict(String),
}

impl From<psgc_application::errors::UsecaseError> for APIError {
    fn from(error: psgc_application::errors::UsecaseError) -> Self {
        match error {
            psgc_application::errors::UsecaseError::NotFound => APIError::NotFound,
            psgc_application::errors::UsecaseError::ValidationError(msg) => {
                APIError::ValidationError(msg)
            }
            psgc_application::errors::UsecaseError::DatabaseError(msg)
            | psgc_application::errors::UsecaseError::UnexpectedError(msg) => {
                APIError::InternalError(msg)
            }
            psgc_application::errors::UsecaseError::Forbidden => APIError::Forbidden,
            psgc_application::errors::UsecaseError::Unauthorized => APIError::Unauthorized,
            psgc_application::errors::UsecaseError::ConstraintViolation(msg)
            | psgc_application::errors::UsecaseError::Conflict(msg) => APIError::Conflict(msg),
        }
    }
}

use std::fmt::Display;

use actix_web::{ResponseError, http::StatusCode};
use psgc_application::errors::UsecaseError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::errors::APIError;

#[derive(Debug, Deserialize, Serialize, ToSchema, bon::Builder)]
pub struct APIOk<T: Serialize> {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[allow(unused)]
impl<T: Serialize> APIOk<T> {
    pub fn success(data: T) -> Self {
        Self::builder()
            .message("Success".to_string())
            .maybe_data(Some(data))
            .build()
    }

    pub fn success_with_message(message: String, data: T) -> Self {
        Self::builder()
            .message(message)
            .maybe_data(Some(data))
            .build()
    }

    pub fn empty() -> Self {
        Self::builder()
            .message("Success".to_string())
            .maybe_data(None)
            .build()
    }

    pub fn empty_with_message(message: String) -> Self {
        Self::builder().message(message).maybe_data(None).build()
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema, bon::Builder)]
pub struct APIErr {
    pub error: APIError,
    pub code: String,
}

impl Display for APIErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use APIError::*;

        match &self.error {
            NotFound => write!(f, "Not found."),
            ValidationError(e) => write!(f, "Validation Error: {e}"),
            InternalError(e) => write!(f, "Something went wrong. {e}"),
            Forbidden => write!(f, "Forbidden access."),
            Unauthorized => write!(f, "Unauthorized access."),
            Conflict(e) => write!(f, "Resource conflict. {e}"),
        }
    }
}

impl ResponseError for APIErr {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use APIError::*;

        match self.error {
            NotFound => StatusCode::NOT_FOUND,
            ValidationError(_) => StatusCode::BAD_REQUEST,
            InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Forbidden => StatusCode::FORBIDDEN,
            Unauthorized => StatusCode::UNAUTHORIZED,
            Conflict(_) => StatusCode::CONFLICT,
        }
    }
}

impl From<UsecaseError> for APIErr {
    fn from(value: UsecaseError) -> Self {
        match value {
            UsecaseError::NotFound => APIErr::builder()
                .code("psgc-404".to_string())
                .error(APIError::NotFound)
                .build(),
            UsecaseError::ValidationError(msg) => APIErr::builder()
                .code("psgc-400".to_string())
                .error(APIError::ValidationError(msg))
                .build(),
            UsecaseError::DatabaseError(msg) => APIErr::builder()
                .code("psgc-500".to_string())
                .error(APIError::InternalError(msg))
                .build(),
            UsecaseError::UnexpectedError(msg) => APIErr::builder()
                .code("psgc-500".to_string())
                .error(APIError::InternalError(msg))
                .build(),
            UsecaseError::Forbidden => APIErr::builder()
                .code("psgc-403".to_string())
                .error(APIError::Forbidden)
                .build(),
            UsecaseError::Unauthorized => APIErr::builder()
                .code("psgc-401".to_string())
                .error(APIError::Unauthorized)
                .build(),
            UsecaseError::ConstraintViolation(msg) => APIErr::builder()
                .code("psgc-409".to_string())
                .error(APIError::Conflict(msg))
                .build(),
            UsecaseError::Conflict(msg) => APIErr::builder()
                .code("psgc-409".to_string())
                .error(APIError::Conflict(msg))
                .build(),
        }
    }
}

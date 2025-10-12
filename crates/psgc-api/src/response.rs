use std::fmt::Display;

use actix_web::{ResponseError, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::errors::APIError;

#[derive(Debug, Deserialize, Serialize, ToSchema, bon::Builder)]
pub struct APIOk<T: Serialize> {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, bon::Builder)]
pub struct APIErr {
    error: APIError,
    code: String,
}

impl Display for APIErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use APIError::*;

        match &self.error {
            NotFound => write!(f, "Not found."),
            ValidationError(e) => write!(f, "Valdiation Error: {e}"),
            InternalError(e) => write!(f, "Something went wrong. {e}"),
            Forbidden => write!(f, "Forbidden access."),
            Unauthorized => write!(f, "Unauthorized access."),
            Conflict(e) => write!(f, "Resource confflict. {e}"),
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
            Conflict(_) => StatusCode::BAD_REQUEST,
        }
    }
}

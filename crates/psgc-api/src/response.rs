use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct APIOk<T: Serialize> {
    message: String,
    data: T,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct APIErr {
    error: String,
    code: String,
}

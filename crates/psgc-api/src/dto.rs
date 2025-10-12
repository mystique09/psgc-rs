use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PaginateQueryParam {
    page: Option<u64>,
    limit: Option<u64>,
}

impl PaginateQueryParam {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(0)
    }

    pub fn limit(&self) -> u64 {
        self.limit.unwrap_or(10).clamp(0, 100)
    }
}

use actix_web::web::Json;
use psgc_application::dto::region_dto::RegionDTO;

use crate::response::{APIErr, APIOk};

#[derive(Debug, utoipa::OpenApi)]
pub struct RegionAPIDoc;

pub async fn list_regions() -> Result<Json<APIOk<Vec<RegionDTO>>>, APIErr> {
    todo!()
}

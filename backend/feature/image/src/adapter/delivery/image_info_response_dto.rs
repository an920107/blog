use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ImageInfoResponseDto {
    pub id: i32,
    pub mime_type: String,
}

use serde::Serialize;

#[derive(Serialize)]
pub struct ImageInfoResponseDto {
    pub id: i32,
    pub mime_type: String,
}

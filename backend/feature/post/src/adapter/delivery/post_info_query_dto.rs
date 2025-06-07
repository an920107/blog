use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostQueryDto {
    pub is_published_only: Option<bool>,
}

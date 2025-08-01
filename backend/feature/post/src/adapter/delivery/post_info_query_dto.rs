use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct PostQueryDto {
    #[param(default = true)]
    pub is_published_only: Option<bool>,
}

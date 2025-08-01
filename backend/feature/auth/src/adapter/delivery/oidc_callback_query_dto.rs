use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct OidcCallbackQueryDto {
    pub code: String,
    pub state: String,
}

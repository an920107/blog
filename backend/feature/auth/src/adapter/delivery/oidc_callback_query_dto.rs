use serde::Deserialize;

#[derive(Deserialize)]
pub struct OidcCallbackQueryDto {
    pub code: String,
    pub state: String,
}

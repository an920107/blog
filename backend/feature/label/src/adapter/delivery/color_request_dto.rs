use serde::Deserialize;
use utoipa::ToSchema;

use crate::domain::entity::color::Color;

#[derive(Deserialize, ToSchema)]
pub struct ColorRequestDto {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl From<ColorRequestDto> for Color {
    fn from(val: ColorRequestDto) -> Self {
        Color {
            red: val.red,
            green: val.green,
            blue: val.blue,
            alpha: val.alpha,
        }
    }
}

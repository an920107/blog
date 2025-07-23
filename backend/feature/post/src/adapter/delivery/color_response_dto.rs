use serde::Serialize;

use crate::domain::entity::color::Color;

#[derive(Serialize)]
pub struct ColorResponseDto {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl From<Color> for ColorResponseDto {
    fn from(color: Color) -> Self {
        Self {
            red: color.red,
            green: color.green,
            blue: color.blue,
            alpha: color.alpha,
        }
    }
}

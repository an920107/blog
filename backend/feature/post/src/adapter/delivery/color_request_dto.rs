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

impl ColorRequestDto {
    pub fn into_entity(self) -> Color {
        Color {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: self.alpha,
        }
    }
}

use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::entity::color::Color;

#[derive(Serialize, ToSchema)]
pub struct ColorResponseDto {
    #[schema(maximum = 255)]
    pub red: u8,

    #[schema(maximum = 255)]
    pub green: u8,

    #[schema(maximum = 255)]
    pub blue: u8,

    #[schema(maximum = 255)]
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

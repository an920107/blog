use serde::Serialize;

use crate::{
    adapter::delivery::color_response_dto::ColorResponseDto, domain::entity::label::Label,
};

#[derive(Serialize)]
pub struct LabelResponseDto {
    pub id: i32,
    pub name: String,
    pub color: ColorResponseDto,
}

impl From<Label> for LabelResponseDto {
    fn from(entity: Label) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            color: ColorResponseDto::from(entity.color),
        }
    }
}

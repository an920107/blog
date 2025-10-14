use serde::Deserialize;
use utoipa::ToSchema;

use crate::{adapter::delivery::color_request_dto::ColorRequestDto, domain::entity::label::Label};

#[derive(Deserialize, ToSchema)]
pub struct UpdateLabelRequestDto {
    pub name: String,
    pub color: ColorRequestDto,
}

impl UpdateLabelRequestDto {
    pub fn into_entity(self, id: i32) -> Label {
        Label {
            id,
            name: self.name,
            color: self.color.into_entity(),
        }
    }
}

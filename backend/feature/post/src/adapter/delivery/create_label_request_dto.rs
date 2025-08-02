use serde::Deserialize;
use utoipa::ToSchema;

use crate::{adapter::delivery::color_request_dto::ColorRequestDto, domain::entity::label::Label};

#[derive(Deserialize, ToSchema)]
pub struct CreateLabelRequestDto {
    pub name: String,
    pub color: ColorRequestDto,
}

impl CreateLabelRequestDto {
    pub fn into_entity(self) -> Label {
        Label {
            id: -1,
            name: self.name,
            color: self.color.into_entity(),
        }
    }
}

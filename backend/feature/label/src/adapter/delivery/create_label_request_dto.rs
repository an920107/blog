use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    adapter::delivery::color_request_dto::ColorRequestDto,
    application::gateway::create_or_update_label_params::CreateOrUpdateLabelParams,
};

#[derive(Deserialize, ToSchema)]
pub struct CreateLabelRequestDto {
    pub name: String,
    pub color: ColorRequestDto,
}

impl From<CreateLabelRequestDto> for CreateOrUpdateLabelParams {
    fn from(val: CreateLabelRequestDto) -> Self {
        CreateOrUpdateLabelParams {
            name: val.name,
            color: val.color.into(),
        }
    }
}

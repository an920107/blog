use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    adapter::delivery::color_request_dto::ColorRequestDto,
    application::gateway::create_or_update_label_params::CreateOrUpdateLabelParams,
};

#[derive(Deserialize, ToSchema)]
pub struct UpdateLabelRequestDto {
    pub name: String,
    pub color: ColorRequestDto,
}

impl From<UpdateLabelRequestDto> for CreateOrUpdateLabelParams {
    fn from(val: UpdateLabelRequestDto) -> Self {
        CreateOrUpdateLabelParams {
            name: val.name,
            color: val.color.into(),
        }
    }
}

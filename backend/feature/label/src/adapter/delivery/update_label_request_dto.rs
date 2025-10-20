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

impl Into<CreateOrUpdateLabelParams> for UpdateLabelRequestDto {
    fn into(self) -> CreateOrUpdateLabelParams {
        CreateOrUpdateLabelParams {
            name: self.name,
            color: self.color.into(),
        }
    }
}

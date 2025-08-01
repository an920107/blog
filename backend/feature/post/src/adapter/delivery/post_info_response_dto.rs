use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::entity::post_info::PostInfo;

use super::label_response_dto::LabelResponseDto;

#[derive(Serialize, ToSchema)]
pub struct PostInfoResponseDto {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub labels: Vec<LabelResponseDto>,

    #[schema(format = Uri)]
    pub preview_image_url: String,

    #[schema(format = DateTime)]
    pub published_time: Option<String>,
}

impl From<PostInfo> for PostInfoResponseDto {
    fn from(entity: PostInfo) -> Self {
        Self {
            id: entity.id,
            title: entity.title,
            description: entity.description,
            preview_image_url: entity.preview_image_url,
            labels: entity
                .labels
                .into_iter()
                .map(LabelResponseDto::from)
                .collect(),
            published_time: entity.published_time.map(|datetime| datetime.to_rfc3339()),
        }
    }
}

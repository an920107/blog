use label::adapter::delivery::label_response_dto::LabelResponseDto;
use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::entity::post_info::PostInfo;

#[derive(Serialize, ToSchema)]
pub struct PostInfoResponseDto {
    pub id: i32,
    pub semantic_id: String,
    pub title: String,
    pub description: String,
    pub labels: Vec<LabelResponseDto>,

    #[schema(format = Uri)]
    pub preview_image_url: Option<String>,

    pub preview_image_mime_type: Option<String>,

    pub preview_image_size: Option<i64>,

    #[schema(format = DateTime)]
    pub published_time: Option<String>,

    #[schema(format = DateTime)]
    pub updated_time: Option<String>,
}

impl From<PostInfo> for PostInfoResponseDto {
    fn from(entity: PostInfo) -> Self {
        Self {
            id: entity.id,
            semantic_id: entity.semantic_id,
            title: entity.title,
            description: entity.description,
            preview_image_url: entity.preview_image_url,
            preview_image_mime_type: entity.preview_image_mime_type,
            preview_image_size: entity.preview_image_size,
            labels: entity
                .labels
                .into_iter()
                .map(LabelResponseDto::from)
                .collect(),
            published_time: entity.published_time.map(|datetime| datetime.to_rfc3339()),
            updated_time: entity.updated_time.map(|datetime| datetime.to_rfc3339()),
        }
    }
}

use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    application::gateway::create_post_params::CreatePostParams,
    application::service::image_extractor::ImageExtractor,
};

#[derive(Deserialize, ToSchema, Clone)]
pub struct CreatePostRequestDto {
    pub semantic_id: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub label_ids: Vec<i32>,

    #[schema(required, format = Uri)]
    pub preview_image_url: Option<String>,

    #[schema(required, format = DateTime)]
    pub published_time: Option<String>,
}

impl From<CreatePostRequestDto> for CreatePostParams {
    fn from(val: CreatePostRequestDto) -> Self {
        let image_ids = ImageExtractor::extract_image_ids(&val.content, &val.preview_image_url);

        CreatePostParams {
            semantic_id: val.semantic_id,
            title: val.title,
            description: val.description,
            content: val.content,
            label_ids: val.label_ids,
            preview_image_url: val.preview_image_url,
            published_time: val
                .published_time
                .and_then(|time_str| DateTime::parse_from_rfc3339(&time_str).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            image_ids,
        }
    }
}

use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::application::gateway::update_post_params::UpdatePostParams;

#[derive(Deserialize, ToSchema, Clone)]
pub struct UpdatePostRequestDto {
    pub title: String,
    pub description: String,
    pub content: String,
    pub label_ids: Vec<i32>,

    #[schema(required, format = Uri)]
    pub preview_image_url: Option<String>,

    #[schema(required, format = DateTime)]
    pub published_time: Option<String>,
}

impl Into<UpdatePostParams> for UpdatePostRequestDto {
    fn into(self) -> UpdatePostParams {
        UpdatePostParams {
            title: self.title,
            description: self.description,
            content: self.content,
            label_ids: self.label_ids,
            preview_image_url: self.preview_image_url,
            published_time: self
                .published_time
                .and_then(|time_str| DateTime::parse_from_rfc3339(&time_str).ok())
                .map(|dt| dt.with_timezone(&Utc)),
        }
    }
}

use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::domain::entity::{post::Post, post_info::PostInfo};

#[derive(Deserialize, ToSchema, Clone)]
pub struct CreatePostRequestDto {
    pub semantic_id: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub label_ids: Vec<i32>,

    #[schema(format = Uri)]
    pub preview_image_url: String,

    #[schema(required, format = DateTime)]
    pub published_time: Option<String>,
}

impl CreatePostRequestDto {
    pub fn into_entity(self) -> Post {
        Post {
            id: -1,
            info: PostInfo {
                id: -1,
                semantic_id: self.semantic_id,
                title: self.title,
                description: self.description,
                preview_image_url: self.preview_image_url,
                labels: Vec::new(),
                published_time: self
                    .published_time
                    .and_then(|time_str| DateTime::parse_from_rfc3339(&time_str).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
            },
            content: self.content,
        }
    }
}

use chrono::{DateTime, Utc};

use crate::domain::{entity::post_info::PostInfo, error::post_error::PostError};

#[derive(Clone)]
pub struct CreatePostParams {
    pub semantic_id: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub label_ids: Vec<i32>,
    pub preview_image_url: Option<String>,
    pub published_time: Option<DateTime<Utc>>,
    pub image_ids: Vec<i32>,
}

impl CreatePostParams {
    pub fn validate(&self) -> Result<(), PostError> {
        PostInfo::validate_semantic_id(&self.semantic_id)
    }
}

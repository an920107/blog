use chrono::{DateTime, Utc};
use label::domain::entity::label::Label;
use regex::Regex;

use crate::domain::error::post_error::PostError;

pub struct PostInfo {
    pub id: i32,
    pub semantic_id: String,
    pub title: String,
    pub description: String,
    pub preview_image_url: Option<String>,
    pub labels: Vec<Label>,
    pub published_time: Option<DateTime<Utc>>,
}

impl PostInfo {
    pub fn validate_semantic_id(semantic_id: &str) -> Result<(), PostError> {
        if semantic_id.parse::<i32>().is_ok() {
            return Err(PostError::InvalidSemanticId);
        }

        let re = Regex::new(r"^[0-9a-zA-Z_-]+$").unwrap();
        if !re.is_match(semantic_id) {
            return Err(PostError::InvalidSemanticId);
        }

        Ok(())
    }
}

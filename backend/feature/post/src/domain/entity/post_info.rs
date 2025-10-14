use chrono::{DateTime, Utc};
use label::domain::entity::label::Label;
use regex::Regex;

use crate::application::error::post_error::PostError;

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
    pub fn validate(&self) -> Result<(), PostError> {
        if self.semantic_id.parse::<i32>().is_ok() {
            return Err(PostError::InvalidSemanticId);
        }

        let re = Regex::new(r"^[0-9a-zA-Z_-]+$").unwrap();
        if !re.is_match(&self.semantic_id) {
            return Err(PostError::InvalidSemanticId);
        }

        Ok(())
    }
}

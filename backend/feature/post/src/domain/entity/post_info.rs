use chrono::{DateTime, Utc};

use super::label::Label;

pub struct PostInfo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub preview_image_url: String,
    pub labels: Vec<Label>,
    pub published_time: Option<DateTime<Utc>>,
}

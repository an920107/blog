use chrono::{DateTime, Utc};

pub struct UpdatePostParams {
    pub title: String,
    pub description: String,
    pub content: String,
    pub label_ids: Vec<i32>,
    pub preview_image_url: Option<String>,
    pub published_time: Option<DateTime<Utc>>,
}

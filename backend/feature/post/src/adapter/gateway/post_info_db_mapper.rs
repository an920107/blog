use chrono::{DateTime, NaiveDateTime, Utc};
use label::adapter::gateway::label_db_mapper::LabelMapper;

use crate::domain::entity::post_info::PostInfo;

pub struct PostInfoMapper {
    pub id: i32,
    pub semantic_id: String,
    pub title: String,
    pub description: String,
    pub preview_image_url: Option<String>,
    pub published_time: Option<NaiveDateTime>,
    pub labels: Vec<LabelMapper>,
}

impl Into<PostInfo> for PostInfoMapper {
    fn into(self) -> PostInfo {
        PostInfo {
            id: self.id,
            semantic_id: self.semantic_id,
            title: self.title,
            description: self.description,
            preview_image_url: self.preview_image_url,
            published_time: self
                .published_time
                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)),
            labels: self.labels.into_iter().map(Into::into).collect(),
        }
    }
}

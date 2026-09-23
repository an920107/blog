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
    pub updated_time: Option<NaiveDateTime>,
    pub labels: Vec<LabelMapper>,
}

impl From<PostInfoMapper> for PostInfo {
    fn from(val: PostInfoMapper) -> Self {
        PostInfo {
            id: val.id,
            semantic_id: val.semantic_id,
            title: val.title,
            description: val.description,
            preview_image_url: val.preview_image_url,
            // Filled in by the use case from the image feature when needed.
            preview_image_mime_type: None,
            preview_image_size: None,
            published_time: val
                .published_time
                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)),
            updated_time: val
                .updated_time
                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)),
            labels: val.labels.into_iter().map(Into::into).collect(),
        }
    }
}

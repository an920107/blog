use chrono::{DateTime, NaiveDateTime, Utc};

use crate::{adapter::gateway::label_db_mapper::LabelMapper, domain::entity::post_info::PostInfo};

pub struct PostInfoMapper {
    pub id: i32,
    pub semantic_id: String,
    pub title: String,
    pub description: String,
    pub preview_image_url: String,
    pub published_time: Option<NaiveDateTime>,
    pub labels: Vec<LabelMapper>,
}

impl PostInfoMapper {
    pub fn into_entity(self) -> PostInfo {
        PostInfo {
            id: self.id,
            semantic_id: self.semantic_id,
            title: self.title.clone(),
            description: self.description.clone(),
            preview_image_url: self.preview_image_url.clone(),
            published_time: self
                .published_time
                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)),
            labels: self
                .labels
                .into_iter()
                .map(LabelMapper::into_entity)
                .collect(),
        }
    }
}

use async_trait::async_trait;

use crate::{
    adapter::gateway::label_db_mapper::LabelMapper, application::error::post_error::PostError,
};

#[async_trait]
pub trait LabelDbService: Send + Sync {
    async fn create_label(&self, label: LabelMapper) -> Result<i32, PostError>;
    async fn update_label(&self, label: LabelMapper) -> Result<(), PostError>;
    async fn get_label_by_id(&self, id: i32) -> Result<LabelMapper, PostError>;
    async fn get_all_labels(&self) -> Result<Vec<LabelMapper>, PostError>;
}

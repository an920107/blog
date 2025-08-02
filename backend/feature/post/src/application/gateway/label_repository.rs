use async_trait::async_trait;

use crate::{application::error::post_error::PostError, domain::entity::label::Label};

#[async_trait]
pub trait LabelRepository: Send + Sync {
    async fn create_label(&self, label: Label) -> Result<i32, PostError>;
    async fn update_label(&self, label: Label) -> Result<(), PostError>;
    async fn get_label_by_id(&self, id: i32) -> Result<Label, PostError>;
    async fn get_all_labels(&self) -> Result<Vec<Label>, PostError>;
}

use async_trait::async_trait;

use crate::{
    adapter::gateway::dto::label_db_model::LabelDbModel, domain::error::label_error::LabelError,
};

#[async_trait]
pub trait LabelDbService: Send + Sync {
    async fn create_label(&self, label: LabelDbModel) -> Result<i32, LabelError>;
    async fn update_label(&self, label: LabelDbModel) -> Result<(), LabelError>;
    async fn get_label_by_id(&self, id: i32) -> Result<LabelDbModel, LabelError>;
    async fn get_all_labels(&self) -> Result<Vec<LabelDbModel>, LabelError>;
}

use async_trait::async_trait;

use crate::{
    application::gateway::create_or_update_label_params::CreateOrUpdateLabelParams,
    domain::{entity::label::Label, error::label_error::LabelError},
};

#[async_trait]
pub trait LabelRepository: Send + Sync {
    async fn create_label(&self, label: CreateOrUpdateLabelParams) -> Result<i32, LabelError>;
    async fn update_label(
        &self,
        id: i32,
        label: CreateOrUpdateLabelParams,
    ) -> Result<(), LabelError>;
    async fn get_label_by_id(&self, id: i32) -> Result<Label, LabelError>;
    async fn get_all_labels(&self) -> Result<Vec<Label>, LabelError>;
}

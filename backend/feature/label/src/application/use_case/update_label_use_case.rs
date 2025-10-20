use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::gateway::{
        create_or_update_label_params::CreateOrUpdateLabelParams, label_repository::LabelRepository,
    },
    domain::error::label_error::LabelError,
};

#[async_trait]
pub trait UpdateLabelUseCase: Send + Sync {
    async fn execute(&self, id: i32, label: CreateOrUpdateLabelParams) -> Result<(), LabelError>;
}

pub struct UpdateLabelUseCaseImpl {
    label_repository: Arc<dyn LabelRepository>,
}

impl UpdateLabelUseCaseImpl {
    pub fn new(label_repository: Arc<dyn LabelRepository>) -> Self {
        Self { label_repository }
    }
}

#[async_trait]
impl UpdateLabelUseCase for UpdateLabelUseCaseImpl {
    async fn execute(&self, id: i32, label: CreateOrUpdateLabelParams) -> Result<(), LabelError> {
        self.label_repository.update_label(id, label).await
    }
}

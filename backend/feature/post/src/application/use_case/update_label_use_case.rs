use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::{error::post_error::PostError, gateway::label_repository::LabelRepository},
    domain::entity::label::Label,
};

#[async_trait]
pub trait UpdateLabelUseCase: Send + Sync {
    async fn execute(&self, label: Label) -> Result<(), PostError>;
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
    async fn execute(&self, label: Label) -> Result<(), PostError> {
        self.label_repository.update_label(label).await
    }
}

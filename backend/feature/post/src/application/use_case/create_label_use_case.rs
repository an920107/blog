use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::{error::post_error::PostError, gateway::label_repository::LabelRepository},
    domain::entity::label::Label,
};

#[async_trait]
pub trait CreateLabelUseCase: Send + Sync {
    async fn execute(&self, label: Label) -> Result<i32, PostError>;
}

pub struct CreateLabelUseCaseImpl {
    label_repository: Arc<dyn LabelRepository>,
}

impl CreateLabelUseCaseImpl {
    pub fn new(label_repository: Arc<dyn LabelRepository>) -> Self {
        Self { label_repository }
    }
}

#[async_trait]
impl CreateLabelUseCase for CreateLabelUseCaseImpl {
    async fn execute(&self, label: Label) -> Result<i32, PostError> {
        self.label_repository.create_label(label).await
    }
}

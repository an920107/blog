use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::{error::post_error::PostError, gateway::label_repository::LabelRepository},
    domain::entity::label::Label,
};

#[async_trait]
pub trait GetAllLabelsUseCase: Send + Sync {
    async fn execute(&self) -> Result<Vec<Label>, PostError>;
}

pub struct GetAllLabelsUseCaseImpl {
    label_repository: Arc<dyn LabelRepository>,
}

impl GetAllLabelsUseCaseImpl {
    pub fn new(label_repository: Arc<dyn LabelRepository>) -> Self {
        Self { label_repository }
    }
}

#[async_trait]
impl GetAllLabelsUseCase for GetAllLabelsUseCaseImpl {
    async fn execute(&self) -> Result<Vec<Label>, PostError> {
        self.label_repository.get_all_labels().await
    }
}

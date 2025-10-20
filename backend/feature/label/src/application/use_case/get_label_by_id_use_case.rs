use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::gateway::label_repository::LabelRepository,
    domain::{entity::label::Label, error::label_error::LabelError},
};

#[async_trait]
pub trait GetLabelByIdUseCase: Send + Sync {
    async fn execute(&self, id: i32) -> Result<Label, LabelError>;
}

pub struct GetLabelByIdUseCaseImpl {
    repository: Arc<dyn LabelRepository>,
}

impl GetLabelByIdUseCaseImpl {
    pub fn new(repository: Arc<dyn LabelRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl GetLabelByIdUseCase for GetLabelByIdUseCaseImpl {
    async fn execute(&self, id: i32) -> Result<Label, LabelError> {
        self.repository.get_label_by_id(id).await
    }
}

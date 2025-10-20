use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::gateway::{
        create_or_update_label_params::CreateOrUpdateLabelParams, label_repository::LabelRepository,
    },
    domain::error::label_error::LabelError,
};

#[async_trait]
pub trait CreateLabelUseCase: Send + Sync {
    async fn execute(&self, label: CreateOrUpdateLabelParams) -> Result<i32, LabelError>;
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
    async fn execute(&self, label: CreateOrUpdateLabelParams) -> Result<i32, LabelError> {
        self.label_repository.create_label(label).await
    }
}

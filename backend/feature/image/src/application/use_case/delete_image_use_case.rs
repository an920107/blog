use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::gateway::image_repository::ImageRepository, domain::error::image_error::ImageError,
};

#[async_trait]
pub trait DeleteImageUseCase: Send + Sync {
    async fn execute(&self, id: i32) -> Result<(), ImageError>;
}

pub struct DeleteImageUseCaseImpl {
    image_repository: Arc<dyn ImageRepository>,
}

impl DeleteImageUseCaseImpl {
    pub fn new(image_repository: Arc<dyn ImageRepository>) -> Self {
        Self { image_repository }
    }
}

#[async_trait]
impl DeleteImageUseCase for DeleteImageUseCaseImpl {
    async fn execute(&self, id: i32) -> Result<(), ImageError> {
        self.image_repository.delete_image(id).await
    }
}

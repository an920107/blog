use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::gateway::image_repository::ImageRepository,
    domain::{entity::image_info::ImageInfo, error::image_error::ImageError},
};

#[async_trait]
pub trait ListImagesUseCase: Send + Sync {
    async fn execute(&self) -> Result<Vec<ImageInfo>, ImageError>;
}

pub struct ListImagesUseCaseImpl {
    image_repository: Arc<dyn ImageRepository>,
}

impl ListImagesUseCaseImpl {
    pub fn new(image_repository: Arc<dyn ImageRepository>) -> Self {
        Self { image_repository }
    }
}

#[async_trait]
impl ListImagesUseCase for ListImagesUseCaseImpl {
    async fn execute(&self) -> Result<Vec<ImageInfo>, ImageError> {
        self.image_repository.list_images().await
    }
}

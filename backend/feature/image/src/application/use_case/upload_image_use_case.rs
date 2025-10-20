use crate::{
    application::gateway::{
        create_image_params::CreateImageParams, image_repository::ImageRepository,
    },
    domain::error::image_error::ImageError,
};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait UploadImageUseCase: Send + Sync {
    async fn execute(&self, image: CreateImageParams) -> Result<i32, ImageError>;
}

pub struct UploadImageUseCaseImpl {
    image_repository: Arc<dyn ImageRepository>,
}

impl UploadImageUseCaseImpl {
    pub fn new(image_repository: Arc<dyn ImageRepository>) -> Self {
        Self { image_repository }
    }
}

#[async_trait]
impl UploadImageUseCase for UploadImageUseCaseImpl {
    async fn execute(&self, image: CreateImageParams) -> Result<i32, ImageError> {
        self.image_repository.save_image(image).await
    }
}

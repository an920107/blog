use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::gateway::image_repository::ImageRepository,
    domain::{entity::image::Image, error::image_error::ImageError},
};

#[async_trait]
pub trait GetImageUseCase: Send + Sync {
    async fn execute(&self, id: i32) -> Result<Image, ImageError>;
}

pub struct GetImageUseCaseImpl {
    image_repository: Arc<dyn ImageRepository>,
}

impl GetImageUseCaseImpl {
    pub fn new(image_repository: Arc<dyn ImageRepository>) -> Self {
        Self { image_repository }
    }
}

#[async_trait]
impl GetImageUseCase for GetImageUseCaseImpl {
    async fn execute(&self, id: i32) -> Result<Image, ImageError> {
        self.image_repository.get_image_by_id(id).await
    }
}

use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::gateway::image_repository::ImageRepository,
    domain::{entity::image_info::ImageInfo, error::image_error::ImageError},
};

#[async_trait]
pub trait GetImageInfoUseCase: Send + Sync {
    async fn execute(&self, id: i32) -> Result<ImageInfo, ImageError>;
}

pub struct GetImageInfoUseCaseImpl {
    image_repository: Arc<dyn ImageRepository>,
}

impl GetImageInfoUseCaseImpl {
    pub fn new(image_repository: Arc<dyn ImageRepository>) -> Self {
        Self { image_repository }
    }
}

#[async_trait]
impl GetImageInfoUseCase for GetImageInfoUseCaseImpl {
    async fn execute(&self, id: i32) -> Result<ImageInfo, ImageError> {
        self.image_repository.get_image_info_by_id(id).await
    }
}

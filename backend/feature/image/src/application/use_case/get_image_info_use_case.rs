use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::gateway::{
        image_info::ImageInfo, image_reference_checker::ImageReferenceChecker,
        image_repository::ImageRepository,
    },
    domain::error::image_error::ImageError,
};

#[async_trait]
pub trait GetImageInfoUseCase: Send + Sync {
    async fn execute(&self, id: i32) -> Result<ImageInfo, ImageError>;
}

pub struct GetImageInfoUseCaseImpl {
    image_repository: Arc<dyn ImageRepository>,
    image_reference_checker: Arc<dyn ImageReferenceChecker>,
}

impl GetImageInfoUseCaseImpl {
    pub fn new(
        image_repository: Arc<dyn ImageRepository>,
        image_reference_checker: Arc<dyn ImageReferenceChecker>,
    ) -> Self {
        Self {
            image_repository,
            image_reference_checker,
        }
    }
}

#[async_trait]
impl GetImageInfoUseCase for GetImageInfoUseCaseImpl {
    async fn execute(&self, id: i32) -> Result<ImageInfo, ImageError> {
        let image_meta_data = self.image_repository.get_image_meta_data_by_id(id).await?;
        let is_referred = self.image_reference_checker.is_image_referenced(id).await?;

        Ok(ImageInfo {
            id: image_meta_data.id,
            mime_type: image_meta_data.mime_type,
            is_referred,
        })
    }
}

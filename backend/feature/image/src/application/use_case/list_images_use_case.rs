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
pub trait ListImagesUseCase: Send + Sync {
    async fn execute(&self) -> Result<Vec<ImageInfo>, ImageError>;
}

pub struct ListImagesUseCaseImpl {
    image_repository: Arc<dyn ImageRepository>,
    image_reference_checker: Arc<dyn ImageReferenceChecker>,
}

impl ListImagesUseCaseImpl {
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
impl ListImagesUseCase for ListImagesUseCaseImpl {
    async fn execute(&self) -> Result<Vec<ImageInfo>, ImageError> {
        let image_meta_data_list = self.image_repository.list_image_meta_data().await?;
        let image_ids: Vec<i32> = image_meta_data_list
            .iter()
            .map(|meta_data| meta_data.id)
            .collect();
        let reference_counts = self
            .image_reference_checker
            .get_reference_counts(&image_ids)
            .await?;

        let result = image_meta_data_list
            .into_iter()
            .map(|meta_data| ImageInfo {
                id: meta_data.id,
                mime_type: meta_data.mime_type,
                is_referred: reference_counts.get(&meta_data.id).copied().unwrap_or(0) > 0,
            })
            .collect();

        Ok(result)
    }
}

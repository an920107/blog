use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapter::delivery::{
        image_info_response_dto::ImageInfoResponseDto, image_request_dto::ImageRequestDto,
        image_response_dto::ImageResponseDto,
    },
    application::use_case::{
        get_image_use_case::GetImageUseCase, upload_image_use_case::UploadImageUseCase,
    },
    domain::error::image_error::ImageError,
};

#[async_trait]
pub trait ImageController: Send + Sync {
    async fn upload_image(
        &self,
        image: ImageRequestDto,
    ) -> Result<ImageInfoResponseDto, ImageError>;

    async fn get_image_by_id(&self, id: i32) -> Result<ImageResponseDto, ImageError>;
}

pub struct ImageControllerImpl {
    upload_image_use_case: Arc<dyn UploadImageUseCase>,
    get_image_use_case: Arc<dyn GetImageUseCase>,

    mime_type_whitelist: Vec<String>,
}

impl ImageControllerImpl {
    pub fn new(
        upload_image_use_case: Arc<dyn UploadImageUseCase>,
        get_image_use_case: Arc<dyn GetImageUseCase>,
    ) -> Self {
        Self {
            upload_image_use_case,
            get_image_use_case,
            mime_type_whitelist: vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
                "image/gif".to_string(),
                "image/webp".to_string(),
            ],
        }
    }
}

#[async_trait]
impl ImageController for ImageControllerImpl {
    async fn upload_image(
        &self,
        image: ImageRequestDto,
    ) -> Result<ImageInfoResponseDto, ImageError> {
        if !self.mime_type_whitelist.contains(&image.mime_type) {
            return Err(ImageError::UnsupportedMimeType(image.mime_type));
        }

        let mime_type = image.mime_type.clone();
        let id = self.upload_image_use_case.execute(image.into()).await?;
        Ok(ImageInfoResponseDto {
            id: id,
            mime_type: mime_type,
        })
    }

    async fn get_image_by_id(&self, id: i32) -> Result<ImageResponseDto, ImageError> {
        let image = self.get_image_use_case.execute(id).await?;
        Ok(ImageResponseDto {
            id: id,
            mime_type: image.mime_type,
            data: image.data,
        })
    }
}

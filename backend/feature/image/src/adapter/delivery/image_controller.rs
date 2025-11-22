use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapter::delivery::{
        image_info_response_dto::ImageInfoResponseDto, image_request_dto::ImageRequestDto,
        image_response_dto::ImageResponseDto,
    },
    application::use_case::{
        get_image_info_use_case::GetImageInfoUseCase, get_image_use_case::GetImageUseCase,
        list_images_use_case::ListImagesUseCase, upload_image_use_case::UploadImageUseCase,
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
    async fn get_image_info(&self, id: i32) -> Result<ImageInfoResponseDto, ImageError>;

    async fn list_images(&self) -> Result<Vec<ImageInfoResponseDto>, ImageError>;
}

pub struct ImageControllerImpl {
    upload_image_use_case: Arc<dyn UploadImageUseCase>,
    get_image_use_case: Arc<dyn GetImageUseCase>,
    get_image_info_use_case: Arc<dyn GetImageInfoUseCase>,
    list_images_use_case: Arc<dyn ListImagesUseCase>,

    mime_type_whitelist: Vec<String>,
}

impl ImageControllerImpl {
    pub fn new(
        upload_image_use_case: Arc<dyn UploadImageUseCase>,
        get_image_use_case: Arc<dyn GetImageUseCase>,
        get_image_info_use_case: Arc<dyn GetImageInfoUseCase>,
        list_images_use_case: Arc<dyn ListImagesUseCase>,
    ) -> Self {
        Self {
            upload_image_use_case,
            get_image_use_case,
            get_image_info_use_case,
            list_images_use_case,
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
            id: image.info.id,
            mime_type: image.info.mime_type,
            data: image.data,
        })
    }

    async fn get_image_info(&self, id: i32) -> Result<ImageInfoResponseDto, ImageError> {
        let image_info = self.get_image_info_use_case.execute(id).await?;
        Ok(ImageInfoResponseDto {
            id: image_info.id,
            mime_type: image_info.mime_type,
        })
    }

    async fn list_images(&self) -> Result<Vec<ImageInfoResponseDto>, ImageError> {
        let image_infos = self.list_images_use_case.execute().await?;
        Ok(image_infos
            .into_iter()
            .map(|info| ImageInfoResponseDto {
                id: info.id,
                mime_type: info.mime_type,
            })
            .collect())
    }
}

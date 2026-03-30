use async_trait::async_trait;

use crate::{
    application::gateway::create_image_params::CreateImageParams,
    domain::{
        entity::{image::Image, image_meta_data::ImageMetaData},
        error::image_error::ImageError,
    },
};

#[async_trait]
pub trait ImageRepository: Send + Sync {
    async fn save_image(&self, image: CreateImageParams) -> Result<i32, ImageError>;
    async fn get_image_by_id(&self, id: i32) -> Result<Image, ImageError>;
    async fn get_image_meta_data_by_id(&self, id: i32) -> Result<ImageMetaData, ImageError>;
    async fn list_image_meta_data(&self) -> Result<Vec<ImageMetaData>, ImageError>;
    async fn delete_image(&self, id: i32) -> Result<(), ImageError>;
}

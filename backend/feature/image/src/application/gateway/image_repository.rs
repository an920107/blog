use async_trait::async_trait;

use crate::{
    application::gateway::create_image_params::CreateImageParams,
    domain::{entity::image::Image, error::image_error::ImageError},
};

#[async_trait]
pub trait ImageRepository: Send + Sync {
    async fn save_image(&self, image: CreateImageParams) -> Result<i32, ImageError>;
    async fn get_image_by_id(&self, id: i32) -> Result<Image, ImageError>;
}

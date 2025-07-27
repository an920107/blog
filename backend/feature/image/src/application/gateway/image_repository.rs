use async_trait::async_trait;

use crate::{application::error::image_error::ImageError, domain::entity::image::Image};

#[async_trait]
pub trait ImageRepository: Send + Sync {
    async fn save_image(&self, image: Image) -> Result<i32, ImageError>;
    async fn get_image_by_id(&self, id: i32) -> Result<Image, ImageError>;
}

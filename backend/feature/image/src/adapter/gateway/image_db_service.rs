use async_trait::async_trait;

use crate::{
    adapter::gateway::image_db_mapper::ImageDbMapper, domain::error::image_error::ImageError,
};

#[async_trait]
pub trait ImageDbService: Send + Sync {
    async fn create_image_info(&self, image: ImageDbMapper) -> Result<i32, ImageError>;
    async fn get_image_info_by_id(&self, id: i32) -> Result<ImageDbMapper, ImageError>;
    async fn list_image_info(&self) -> Result<Vec<ImageDbMapper>, ImageError>;
}

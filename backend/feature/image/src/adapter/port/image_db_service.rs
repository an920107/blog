use async_trait::async_trait;

use crate::{
    adapter::gateway::dto::image_db_model::ImageDbModel, domain::error::image_error::ImageError,
};

#[async_trait]
pub trait ImageDbService: Send + Sync {
    async fn create_image_info(&self, image: ImageDbModel) -> Result<i32, ImageError>;
    async fn get_image_meta_data_by_id(&self, id: i32) -> Result<ImageDbModel, ImageError>;
    async fn get_image_meta_data_by_ids(
        &self,
        ids: &[i32],
    ) -> Result<Vec<ImageDbModel>, ImageError>;
    async fn list_image_meta_data(&self) -> Result<Vec<ImageDbModel>, ImageError>;
    async fn delete_image(&self, id: i32) -> Result<(), ImageError>;
}

use async_trait::async_trait;

use crate::{
    adapter::gateway::{post_db_mapper::PostMapper, post_info_db_mapper::PostInfoMapper},
    application::error::post_error::PostError,
};

#[async_trait]
pub trait PostDbService: Send + Sync {
    async fn get_all_post_info(
        &self,
        is_published_only: bool,
    ) -> Result<Vec<PostInfoMapper>, PostError>;
    async fn get_post_by_id(&self, id: i32) -> Result<PostMapper, PostError>;
    async fn create_post(&self, post: PostMapper, label_ids: &[i32]) -> Result<i32, PostError>;
    async fn update_post(&self, post: PostMapper, label_ids: &[i32]) -> Result<(), PostError>;
}

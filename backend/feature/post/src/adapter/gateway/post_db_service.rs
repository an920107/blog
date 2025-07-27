use async_trait::async_trait;

use crate::{
    adapter::gateway::{post_info_db_mapper::PostInfoMapper, post_db_mapper::PostMapper},
    application::error::post_error::PostError,
};

#[async_trait]
pub trait PostDbService: Send + Sync {
    async fn get_all_post_info(
        &self,
        is_published_only: bool,
    ) -> Result<Vec<PostInfoMapper>, PostError>;
    async fn get_full_post(&self, id: i32) -> Result<PostMapper, PostError>;
}

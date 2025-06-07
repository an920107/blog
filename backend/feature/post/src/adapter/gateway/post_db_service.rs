use async_trait::async_trait;

use crate::{
    application::error::post_error::PostError,
    domain::entity::{post::Post, post_info::PostInfo},
};

#[async_trait]
pub trait PostDbService: Send + Sync {
    async fn get_all_post_info(&self, is_published_only: bool) -> Result<Vec<PostInfo>, PostError>;
    async fn get_full_post(&self, id: i32) -> Result<Post, PostError>;
}

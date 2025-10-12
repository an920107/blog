use async_trait::async_trait;

use crate::{
    application::error::post_error::PostError,
    domain::entity::{post::Post, post_info::PostInfo},
};

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn get_all_post_info(&self, is_published_only: bool) -> Result<Vec<PostInfo>, PostError>;
    async fn get_post_by_id(&self, id: i32) -> Result<Post, PostError>;
    async fn create_post(&self, post: Post, label_ids: &[i32]) -> Result<i32, PostError>;
    async fn update_post(&self, post: Post, label_ids: &[i32]) -> Result<(), PostError>;
    async fn get_id_by_semantic_id(&self, semantic_id: &str) -> Result<i32, PostError>;
}

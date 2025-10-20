use async_trait::async_trait;

use crate::{
    application::gateway::{
        create_post_params::CreatePostParams, update_post_params::UpdatePostParams,
    },
    domain::{
        entity::{post::Post, post_info::PostInfo},
        error::post_error::PostError,
    },
};

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn get_all_post_info(
        &self,
        is_published_only: bool,
        label_id: Option<i32>,
    ) -> Result<Vec<PostInfo>, PostError>;
    async fn get_post_by_id(&self, id: i32) -> Result<Post, PostError>;
    async fn create_post(&self, post: CreatePostParams) -> Result<i32, PostError>;
    async fn update_post(&self, id: i32, post: UpdatePostParams) -> Result<(), PostError>;
    async fn get_id_by_semantic_id(&self, semantic_id: &str) -> Result<i32, PostError>;
}

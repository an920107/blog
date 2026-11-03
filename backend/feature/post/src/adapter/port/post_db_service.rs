use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    adapter::gateway::dto::{
        post_db_model::PostDbModel,
        post_info_db_model::PostInfoDbModel,
        post_write_db_model::{CreatePostDbModel, UpdatePostDbModel},
    },
    domain::error::post_error::PostError,
};

#[async_trait]
pub trait PostDbService: Send + Sync {
    async fn get_all_post_info(
        &self,
        is_published_only: bool,
        label_id: Option<i32>,
    ) -> Result<Vec<PostInfoDbModel>, PostError>;
    async fn get_post_by_id(&self, id: i32) -> Result<PostDbModel, PostError>;
    async fn create_post(&self, post: CreatePostDbModel) -> Result<i32, PostError>;
    async fn update_post(&self, post: UpdatePostDbModel) -> Result<(), PostError>;
    async fn get_id_by_semantic_id(&self, semantic_id: &str) -> Result<i32, PostError>;
    async fn count_by_image_id(&self, image_id: i32) -> Result<i64, PostError>;
    async fn count_by_image_ids(&self, image_ids: &[i32]) -> Result<HashMap<i32, i64>, PostError>;
}

use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::{error::post_error::PostError, gateway::post_repository::PostRepository},
    domain::entity::post::Post,
};

#[async_trait]
pub trait GetPostByIdUseCase: Send + Sync {
    async fn execute(&self, id: i32, user_id: Option<i32>) -> Result<Post, PostError>;
}

pub struct GetFullPostUseCaseImpl {
    post_repository: Arc<dyn PostRepository>,
}

impl GetFullPostUseCaseImpl {
    pub fn new(post_repository: Arc<dyn PostRepository>) -> Self {
        Self { post_repository }
    }
}

#[async_trait]
impl GetPostByIdUseCase for GetFullPostUseCaseImpl {
    async fn execute(&self, id: i32, user_id: Option<i32>) -> Result<Post, PostError> {
        let post = self.post_repository.get_post_by_id(id).await?;

        if post.info.published_time.is_none() && user_id.is_none() {
            return Err(PostError::Unauthorized);
        }

        Ok(post)
    }
}

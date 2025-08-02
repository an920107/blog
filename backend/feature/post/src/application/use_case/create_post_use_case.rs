use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::{error::post_error::PostError, gateway::post_repository::PostRepository},
    domain::entity::post::Post,
};

#[async_trait]
pub trait CreatePostUseCase: Send + Sync {
    async fn execute(&self, post: Post, label_ids: &[i32]) -> Result<i32, PostError>;
}

pub struct CreatePostUseCaseImpl {
    post_repository: Arc<dyn PostRepository>,
}

impl CreatePostUseCaseImpl {
    pub fn new(post_repository: Arc<dyn PostRepository>) -> Self {
        Self { post_repository }
    }
}

#[async_trait]
impl CreatePostUseCase for CreatePostUseCaseImpl {
    async fn execute(&self, post: Post, label_ids: &[i32]) -> Result<i32, PostError> {
        self.post_repository
            .create_post(post, label_ids)
            .await
    }
}

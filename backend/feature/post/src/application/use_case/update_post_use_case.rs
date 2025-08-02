use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::{error::post_error::PostError, gateway::post_repository::PostRepository},
    domain::entity::post::Post,
};

#[async_trait]
pub trait UpdatePostUseCase: Send + Sync {
    async fn execute(&self, post: Post, label_ids: &[i32]) -> Result<(), PostError>;
}

pub struct UpdatePostUseCaseImpl {
    post_repository: Arc<dyn PostRepository>,
}

impl UpdatePostUseCaseImpl {
    pub fn new(post_repository: Arc<dyn PostRepository>) -> Self {
        Self { post_repository }
    }
}

#[async_trait]
impl UpdatePostUseCase for UpdatePostUseCaseImpl {
    async fn execute(&self, post: Post, label_ids: &[i32]) -> Result<(), PostError> {
        self.post_repository.update_post(post, label_ids).await
    }
}

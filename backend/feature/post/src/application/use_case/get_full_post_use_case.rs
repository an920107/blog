use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::{error::post_error::PostError, gateway::post_repository::PostRepository},
    domain::entity::post::Post,
};

#[async_trait]
pub trait GetFullPostUseCase: Send + Sync {
    async fn execute(&self, id: i32) -> Result<Post, PostError>;
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
impl GetFullPostUseCase for GetFullPostUseCaseImpl {
    async fn execute(&self, id: i32) -> Result<Post, PostError> {
        self.post_repository.get_post_by_id(id).await
    }
}

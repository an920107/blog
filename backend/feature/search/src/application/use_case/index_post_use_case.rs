use std::sync::Arc;

use async_trait::async_trait;
use post::domain::entity::post::Post;

use crate::{
    application::gateway::search_repository::SearchRepository,
    domain::error::search_error::SearchError,
};

#[async_trait]
pub trait IndexPostUseCase: Send + Sync {
    async fn execute(&self, post: Post) -> Result<(), SearchError>;
}

pub struct IndexPostUseCaseImpl {
    search_repository: Arc<dyn SearchRepository>,
}

impl IndexPostUseCaseImpl {
    pub fn new(search_repository: Arc<dyn SearchRepository>) -> Self {
        Self { search_repository }
    }
}

#[async_trait]
impl IndexPostUseCase for IndexPostUseCaseImpl {
    async fn execute(&self, post: Post) -> Result<(), SearchError> {
        self.search_repository.index_post(post).await
    }
}

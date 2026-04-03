use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::gateway::search_repository::SearchRepository,
    domain::error::search_error::SearchError,
};

#[async_trait]
pub trait SearchPostsUseCase: Send + Sync {
    async fn execute(
        &self,
        keyword: &str,
        scope: &Option<Vec<i32>>,
    ) -> Result<Vec<i32>, SearchError>;
}

pub struct SearchPostsUseCaseImpl {
    search_repository: Arc<dyn SearchRepository>,
}

impl SearchPostsUseCaseImpl {
    pub fn new(search_repository: Arc<dyn SearchRepository>) -> Self {
        Self { search_repository }
    }
}

#[async_trait]
impl SearchPostsUseCase for SearchPostsUseCaseImpl {
    async fn execute(
        &self,
        keyword: &str,
        scope: &Option<Vec<i32>>,
    ) -> Result<Vec<i32>, SearchError> {
        self.search_repository.search_posts(keyword, scope).await
    }
}

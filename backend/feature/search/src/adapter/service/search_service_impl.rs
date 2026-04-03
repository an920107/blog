use std::sync::Arc;

use async_trait::async_trait;
use post::{
    application::gateway::search_service::SearchService,
    domain::{entity::post::Post, error::post_error::PostError},
};

use crate::application::use_case::{
    index_post_use_case::IndexPostUseCase, search_posts_use_case::SearchPostsUseCase,
};

pub struct SearchServiceImpl {
    index_post_use_case: Arc<dyn IndexPostUseCase>,
    query_posts_use_case: Arc<dyn SearchPostsUseCase>,
}

impl SearchServiceImpl {
    pub fn new(
        index_post_use_case: Arc<dyn IndexPostUseCase>,
        query_posts_use_case: Arc<dyn SearchPostsUseCase>,
    ) -> Self {
        Self {
            index_post_use_case,
            query_posts_use_case,
        }
    }
}

#[async_trait]
impl SearchService for SearchServiceImpl {
    async fn index_post(&self, post: Post) -> Result<(), PostError> {
        self.index_post_use_case
            .execute(post)
            .await
            .map_err(|e| PostError::Unexpected(anyhow::anyhow!(e)))
    }

    async fn search_posts(
        &self,
        keyword: &str,
        scope: &Option<Vec<i32>>,
    ) -> Result<Vec<i32>, PostError> {
        self.query_posts_use_case
            .execute(keyword, scope)
            .await
            .map_err(|e| PostError::Unexpected(anyhow::anyhow!(e)))
    }
}

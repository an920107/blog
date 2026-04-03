use async_trait::async_trait;
use post::domain::entity::post::Post;

use crate::domain::error::search_error::SearchError;

#[async_trait]
pub trait SearchRepository: Send + Sync {
    async fn index_post(&self, post: Post) -> Result<(), SearchError>;
    async fn search_posts(
        &self,
        keyword: &str,
        scope: &Option<Vec<i32>>,
    ) -> Result<Vec<i32>, SearchError>;
}

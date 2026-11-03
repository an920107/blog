use async_trait::async_trait;

use crate::domain::{entity::post::Post, error::post_error::PostError};

#[async_trait]
pub trait SearchService: Send + Sync {
    async fn index_post(&self, post: Post) -> Result<(), PostError>;
    async fn search_posts(
        &self,
        keyword: &str,
        scope: &Option<Vec<i32>>,
    ) -> Result<Vec<i32>, PostError>;
}

use async_trait::async_trait;

use crate::domain::error::search_error::SearchError;

#[async_trait]
pub trait SearchQueryEmbeddingCache: Send + Sync {
    async fn get_by_query_string(
        &self,
        query_string: &str,
    ) -> Result<Option<Vec<f32>>, SearchError>;

    async fn set_by_query_string(
        &self,
        query_string: &str,
        embedding: &Vec<f32>,
    ) -> Result<(), SearchError>;
}
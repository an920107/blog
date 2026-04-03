use async_trait::async_trait;

use crate::domain::error::search_error::SearchError;

#[async_trait]
pub trait SearchVectorDbService: Send + Sync {
    async fn insert_vectors(
        &self,
        post_id: i32,
        vectors: &Vec<Vec<f32>>,
    ) -> Result<(), SearchError>;

    async fn delete_vectors_by_post_id(&self, post_id: i32) -> Result<(), SearchError>;

    async fn search_similar_posts(
        &self,
        query_vector: &Vec<f32>,
        scope: &Option<Vec<i32>>,
    ) -> Result<Vec<i32>, SearchError>;
}

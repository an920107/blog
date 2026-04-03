use anyhow::anyhow;
use async_trait::async_trait;
use deadpool_redis::Pool;
use fnv_rs::{Fnv128, FnvHasher};
use redis::AsyncCommands;

use crate::{
    adapter::gateway::search_query_embedding_cache::SearchQueryEmbeddingCache,
    domain::error::search_error::SearchError,
};

pub struct SearchQueryEmbeddingCacheRedisImpl {
    redis_pool: Pool,
    key_prefix: String,
    model_revision: String,
}

impl SearchQueryEmbeddingCacheRedisImpl {
    pub fn new(redis_pool: Pool, key_prefix: &str, model_revision: &str) -> Self {
        Self {
            redis_pool,
            key_prefix: key_prefix.to_string(),
            model_revision: model_revision.to_string(),
        }
    }

    fn generate_cache_key(&self, query_string: &str) -> String {
        let hash = Fnv128::hash(query_string.as_bytes());
        format!("{}:{}:{hash:032x}", self.key_prefix, self.model_revision)
    }
}

#[async_trait]
impl SearchQueryEmbeddingCache for SearchQueryEmbeddingCacheRedisImpl {
    async fn get_by_query_string(
        &self,
        query_string: &str,
    ) -> Result<Option<Vec<f32>>, SearchError> {
        let key = self.generate_cache_key(query_string);

        let mut connection = self
            .redis_pool
            .get()
            .await
            .map_err(|e| SearchError::Unexpected(anyhow!(e)))?;

        let raw: Option<String> = connection
            .get(key)
            .await
            .map_err(|e| SearchError::Unexpected(anyhow!(e)))?;

        raw.map(|v| {
            serde_json::from_str::<Vec<f32>>(&v).map_err(|e| SearchError::Unexpected(anyhow!(e)))
        })
        .transpose()
    }

    async fn set_by_query_string(
        &self,
        query_string: &str,
        embedding: &Vec<f32>,
    ) -> Result<(), SearchError> {
        let key = self.generate_cache_key(query_string);
        let value =
            serde_json::to_string(embedding).map_err(|e| SearchError::Unexpected(e.into()))?;

        let mut connection = self
            .redis_pool
            .get()
            .await
            .map_err(|e| SearchError::Unexpected(anyhow!(e)))?;

        connection
            .set::<_, _, ()>(key, value)
            .await
            .map_err(|e| SearchError::Unexpected(anyhow!(e)))
    }
}

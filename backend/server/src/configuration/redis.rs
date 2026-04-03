use deadpool_redis::{Config, Pool, Runtime};

#[derive(Clone)]
pub struct RedisConfiguration {
    pub url: String,
    pub session_prefix: String,
    pub search_query_embedding_prefix: String,
}

impl RedisConfiguration {
    pub fn new() -> Self {
        let url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

        let session_prefix =
            std::env::var("REDIS_SESSION_PREFIX").unwrap_or_else(|_| "session".to_string());
        let search_query_embedding_prefix = std::env::var("REDIS_SEARCH_QUERY_EMBEDDING_PREFIX")
            .unwrap_or_else(|_| "search:query_embedding".to_string());

        Self {
            url,
            session_prefix,
            search_query_embedding_prefix,
        }
    }

    pub fn create_pool(&self) -> Pool {
        let cfg = Config::from_url(self.url.clone());
        cfg.create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create Redis connection pool")
    }
}

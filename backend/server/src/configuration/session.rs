use actix_session::storage::RedisSessionStore;
use actix_web::cookie::Key;
use deadpool_redis::Pool;

#[derive(Clone)]
pub struct SessionConfiguration {
    pub session_key: Key,

    redis_session_prefix: String,
}

impl SessionConfiguration {
    pub fn new(redis_session_prefix: &str) -> Self {
        let session_key_hex = std::env::var("SESSION_KEY").expect("SESSION_KEY must be set");
        let session_key_bytes =
            hex::decode(session_key_hex).expect("Invalid SESSION_KEY format, must be hex encoded");

        if session_key_bytes.len() != 64 {
            panic!("SESSION_KEY must be 64 bytes long");
        }

        let session_key = Key::from(&session_key_bytes);

        Self {
            session_key,
            redis_session_prefix: redis_session_prefix.to_string(),
        }
    }

    pub async fn create_session_store(&self, redis_pool: Pool) -> RedisSessionStore {
        let prefix = self.redis_session_prefix.clone();

        RedisSessionStore::builder_pooled(redis_pool)
            .cache_keygen(move |session_key| format!("{}:{}", prefix, session_key))
            .build()
            .await
            .expect("Failed to create Redis session store")
    }
}

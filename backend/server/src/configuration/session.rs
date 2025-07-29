use actix_session::storage::RedisSessionStore;
use actix_web::cookie::Key;

#[derive(Clone)]
pub struct SessionConfiguration {
    pub session_key: Key,
    pub redis_url: String,
}

impl SessionConfiguration {
    pub fn new() -> Self {
        let session_key_hex = std::env::var("SESSION_KEY").expect("SESSION_KEY must be set");
        let session_key_bytes =
            hex::decode(session_key_hex).expect("Invalid SESSION_KEY format, must be hex encoded");

        if session_key_bytes.len() != 64 {
            panic!("SESSION_KEY must be 64 bytes long");
        }

        let session_key = Key::from(&session_key_bytes);

        let redis_url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.1:6379".to_string());

        Self {
            session_key,
            redis_url,
        }
    }

    pub async fn create_session_store(&self) -> RedisSessionStore {
        RedisSessionStore::new(self.redis_url.clone())
            .await
            .expect("Failed to create Redis session store")
    }
}

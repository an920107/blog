use openidconnect::reqwest;

use crate::configuration::{
    db::DbConfiguration, embedding::EmbeddingConfiguration, oidc::OidcConfiguration,
    qdrant::QdrantConfiguration, redis::RedisConfiguration, sentry::SentryConfiguration,
    server::ServerConfiguration, session::SessionConfiguration, storage::StorageConfiguration,
};

pub mod db;
pub mod embedding;
pub mod oidc;
pub mod qdrant;
pub mod redis;
pub mod sentry;
pub mod server;
pub mod session;
pub mod storage;

#[derive(Clone)]
pub struct Configuration {
    pub db: DbConfiguration,
    pub embedding: EmbeddingConfiguration,
    pub oidc: OidcConfiguration,
    pub qdrant: QdrantConfiguration,
    pub redis: RedisConfiguration,
    pub sentry: SentryConfiguration,
    pub server: ServerConfiguration,
    pub session: SessionConfiguration,
    pub storage: StorageConfiguration,
}

impl Configuration {
    pub async fn new(http_client: reqwest::Client) -> Self {
        let redis = RedisConfiguration::new();
        Self {
            db: DbConfiguration::new(),
            embedding: EmbeddingConfiguration::new(),
            oidc: OidcConfiguration::new(http_client).await,
            qdrant: QdrantConfiguration::new(),
            sentry: SentryConfiguration::new(),
            server: ServerConfiguration::new(),
            session: SessionConfiguration::new(&redis.session_prefix),
            storage: StorageConfiguration::new(),
            redis,
        }
    }
}

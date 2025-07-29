use openidconnect::reqwest;

use crate::configuration::{
    db::DbConfiguration, oidc::OidcConfiguration, server::ServerConfiguration,
    session::SessionConfiguration, storage::StorageConfiguration,
};

pub mod db;
pub mod oidc;
pub mod server;
pub mod session;
pub mod storage;

#[derive(Clone)]
pub struct Configuration {
    pub db: DbConfiguration,
    pub oidc: OidcConfiguration,
    pub server: ServerConfiguration,
    pub session: SessionConfiguration,
    pub storage: StorageConfiguration,
}

impl Configuration {
    pub async fn new(http_client: reqwest::Client) -> Self {
        Self {
            db: DbConfiguration::new(),
            oidc: OidcConfiguration::new(http_client).await,
            server: ServerConfiguration::new(),
            session: SessionConfiguration::new(),
            storage: StorageConfiguration::new(),
        }
    }
}

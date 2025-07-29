use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapter::gateway::auth_oidc_service::AuthOidcService,
    application::{
        error::auth_error::AuthError, gateway::auth_repository::AuthRepository,
        use_case::get_auth_url_use_case::AuthUrl,
    },
    domain::entity::user::User,
};

pub struct AuthRepositoryImpl {
    auth_oidc_service: Arc<dyn AuthOidcService>,
}

impl AuthRepositoryImpl {
    pub fn new(auth_oidc_service: Arc<dyn AuthOidcService>) -> Self {
        Self { auth_oidc_service }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    fn get_auth_url(&self) -> Result<AuthUrl, AuthError> {
        self.auth_oidc_service.get_auth_url()
    }

    async fn exchange_auth_code(
        &self,
        code: &str,
        expected_nonce: &str,
    ) -> Result<User, AuthError> {
        self.auth_oidc_service
            .exchange_auth_code(code, expected_nonce)
            .await
            .map(|dto| dto.into_entity())
    }
}

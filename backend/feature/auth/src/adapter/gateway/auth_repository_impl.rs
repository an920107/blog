use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapter::gateway::{
        auth_oidc_service::AuthOidcService, user_db_mapper::UserMapper,
        user_db_service::UserDbService,
    },
    application::{
        error::auth_error::AuthError, gateway::auth_repository::AuthRepository,
        use_case::get_auth_url_use_case::AuthUrl,
    },
    domain::entity::user::User,
};

pub struct AuthRepositoryImpl {
    user_db_service: Arc<dyn UserDbService>,
    auth_oidc_service: Arc<dyn AuthOidcService>,
}

impl AuthRepositoryImpl {
    pub fn new(
        user_db_service: Arc<dyn UserDbService>,
        auth_oidc_service: Arc<dyn AuthOidcService>,
    ) -> Self {
        Self {
            user_db_service,
            auth_oidc_service,
        }
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

    async fn get_user_by_id(&self, user_id: i32) -> Result<User, AuthError> {
        self.user_db_service
            .get_user_by_id(user_id)
            .await
            .map(|mapper| mapper.into_entity())
    }

    async fn get_user_by_source_id(
        &self,
        issuer: &str,
        source_id: &str,
    ) -> Result<User, AuthError> {
        self.user_db_service
            .get_user_by_source_id(issuer, source_id)
            .await
            .map(|mapper| mapper.into_entity())
    }

    async fn save_user(&self, user: User) -> Result<i32, AuthError> {
        self.user_db_service
            .create_user(UserMapper::from(user))
            .await
    }
}

use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::{error::auth_error::AuthError, gateway::auth_repository::AuthRepository},
    domain::entity::user::User,
};

#[async_trait]
pub trait ExchangeAuthCodeUseCase: Send + Sync {
    async fn execute(
        &self,
        code: &str,
        received_state: &str,
        expected_state: &str,
        expected_nonce: &str,
    ) -> Result<User, AuthError>;
}

pub struct ExchangeAuthCodeUseCaseImpl {
    auth_repository: Arc<dyn AuthRepository>,
}

impl ExchangeAuthCodeUseCaseImpl {
    pub fn new(auth_repository: Arc<dyn AuthRepository>) -> Self {
        Self { auth_repository }
    }
}

#[async_trait]
impl ExchangeAuthCodeUseCase for ExchangeAuthCodeUseCaseImpl {
    async fn execute(
        &self,
        code: &str,
        received_state: &str,
        expected_state: &str,
        expected_nonce: &str,
    ) -> Result<User, AuthError> {
        if received_state != expected_state {
            return Err(AuthError::InvalidState);
        }

        self.auth_repository
            .exchange_auth_code(code, expected_nonce)
            .await
    }
}

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

        let mut logged_in_user = self
            .auth_repository
            .exchange_auth_code(code, expected_nonce)
            .await?;

        let saved_user_result = self
            .auth_repository
            .get_user_by_source_id(&logged_in_user.issuer, &logged_in_user.source_id)
            .await;

        match saved_user_result {
            Ok(user) => {
                logged_in_user.id = user.id;
            }
            Err(AuthError::UserNotFound) => {
                let id = self
                    .auth_repository
                    .save_user(logged_in_user.clone())
                    .await?;
                logged_in_user.id = id;
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(logged_in_user)
    }
}

use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapter::delivery::{
        oidc_callback_query_dto::OidcCallbackQueryDto, user_response_dto::UserResponseDto,
    },
    application::use_case::{
        exchange_auth_code_use_case::ExchangeAuthCodeUseCase,
        get_auth_url_use_case::{AuthUrl, GetAuthUrlUseCase},
        get_user_use_case::GetUserUseCase,
    },
    domain::error::auth_error::AuthError,
};

#[async_trait]
pub trait AuthController: Send + Sync {
    fn oidc_login(&self) -> Result<AuthUrl, AuthError>;

    async fn oidc_callback(
        &self,
        query: OidcCallbackQueryDto,
        expected_state: &str,
        expected_nonce: &str,
    ) -> Result<UserResponseDto, AuthError>;

    async fn get_user(&self, user_id: i32) -> Result<UserResponseDto, AuthError>;
}

pub struct AuthControllerImpl {
    get_auth_url_use_case: Arc<dyn GetAuthUrlUseCase>,
    exchange_auth_code_use_case: Arc<dyn ExchangeAuthCodeUseCase>,
    get_user_use_case: Arc<dyn GetUserUseCase>,
}

impl AuthControllerImpl {
    pub fn new(
        get_auth_url_use_case: Arc<dyn GetAuthUrlUseCase>,
        exchange_auth_code_use_case: Arc<dyn ExchangeAuthCodeUseCase>,
        get_user_use_case: Arc<dyn GetUserUseCase>,
    ) -> Self {
        Self {
            get_auth_url_use_case,
            exchange_auth_code_use_case,
            get_user_use_case,
        }
    }
}

#[async_trait]
impl AuthController for AuthControllerImpl {
    fn oidc_login(&self) -> Result<AuthUrl, AuthError> {
        self.get_auth_url_use_case.execute()
    }

    async fn oidc_callback(
        &self,
        query: OidcCallbackQueryDto,
        expected_state: &str,
        expected_nonce: &str,
    ) -> Result<UserResponseDto, AuthError> {
        let result = self
            .exchange_auth_code_use_case
            .execute(&query.code, &query.state, expected_state, expected_nonce)
            .await;

        result.map(|user| UserResponseDto::from(user))
    }

    async fn get_user(&self, user_id: i32) -> Result<UserResponseDto, AuthError> {
        let user = self.get_user_use_case.execute(user_id).await?;
        Ok(UserResponseDto::from(user))
    }
}

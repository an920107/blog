use async_trait::async_trait;

use crate::{
    application::{error::auth_error::AuthError, use_case::get_auth_url_use_case::AuthUrl},
    domain::entity::user::User,
};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    fn get_auth_url(&self) -> Result<AuthUrl, AuthError>;
    async fn exchange_auth_code(&self, code: &str, expected_nonce: &str)
    -> Result<User, AuthError>;
}

use async_trait::async_trait;

use crate::{
    application::use_case::get_auth_url_use_case::AuthUrl,
    domain::{entity::user::User, error::auth_error::AuthError},
};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    fn get_auth_url(&self) -> Result<AuthUrl, AuthError>;

    async fn exchange_auth_code(&self, code: &str, expected_nonce: &str)
    -> Result<User, AuthError>;

    async fn get_user_by_id(&self, user_id: i32) -> Result<User, AuthError>;

    async fn get_user_by_source_id(&self, issuer: &str, source_id: &str)
    -> Result<User, AuthError>;

    async fn save_user(&self, user: User) -> Result<i32, AuthError>;
}

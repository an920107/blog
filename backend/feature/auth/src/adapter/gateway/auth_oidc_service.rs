use async_trait::async_trait;

use crate::{
    adapter::gateway::oidc_claims_response_dto::OidcClaimsResponseDto,
    application::{error::auth_error::AuthError, use_case::get_auth_url_use_case::AuthUrl},
};

#[async_trait]
pub trait AuthOidcService: Send + Sync {
    fn get_auth_url(&self) -> Result<AuthUrl, AuthError>;
    async fn exchange_auth_code(
        &self,
        code: &str,
        expected_nonce: &str,
    ) -> Result<OidcClaimsResponseDto, AuthError>;
}

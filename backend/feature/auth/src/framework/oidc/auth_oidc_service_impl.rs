use async_trait::async_trait;
use openidconnect::{
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointMaybeSet, EndpointNotSet,
    EndpointSet, Nonce, RedirectUrl, TokenResponse as _,
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
};

use crate::{
    adapter::gateway::{
        auth_oidc_service::AuthOidcService, oidc_claims_response_dto::OidcClaimsResponseDto,
    },
    application::{error::auth_error::AuthError, use_case::get_auth_url_use_case::AuthUrl},
};

type CompleteClient<
    HasAuthUrl = EndpointSet,
    HasDeviceAuthUrl = EndpointNotSet,
    HasIntrospectionUrl = EndpointNotSet,
    HasRevocationUrl = EndpointNotSet,
    HasTokenUrl = EndpointMaybeSet,
    HasUserInfoUrl = EndpointMaybeSet,
> = CoreClient<
    HasAuthUrl,
    HasDeviceAuthUrl,
    HasIntrospectionUrl,
    HasRevocationUrl,
    HasTokenUrl,
    HasUserInfoUrl,
>;

pub struct AuthOidcServiceImpl {
    oidc_client: CompleteClient,
    http_client: openidconnect::reqwest::Client,
}

impl AuthOidcServiceImpl {
    pub fn new(
        provider_metadata: CoreProviderMetadata,
        client_id: &str,
        client_secret: &str,
        redirect_url: RedirectUrl,
        http_client: openidconnect::reqwest::Client,
    ) -> Self {
        Self {
            oidc_client: CoreClient::from_provider_metadata(
                provider_metadata,
                ClientId::new(client_id.to_string()),
                Some(ClientSecret::new(client_secret.to_string())),
            )
            .set_redirect_uri(redirect_url),
            http_client,
        }
    }
}

#[async_trait]
impl AuthOidcService for AuthOidcServiceImpl {
    fn get_auth_url(&self) -> Result<AuthUrl, AuthError> {
        let (url, state, nonce) = self
            .oidc_client
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .url();

        Ok(AuthUrl {
            url: url.to_string(),
            state: state.secret().into(),
            nonce: nonce.secret().into(),
        })
    }

    async fn exchange_auth_code(
        &self,
        code: &str,
        expected_nonce: &str,
    ) -> Result<OidcClaimsResponseDto, AuthError> {
        let token_response = self
            .oidc_client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .map_err(|e| AuthError::OidcError(e.to_string()))?
            .request_async(&self.http_client)
            .await
            .map_err(|_| AuthError::InvalidAuthCode)?;

        let id_token = token_response.id_token().ok_or(AuthError::InvalidIdToken)?;
        let claims = id_token
            .claims(
                &self.oidc_client.id_token_verifier(),
                &Nonce::new(expected_nonce.to_string()),
            )
            .map_err(|_| AuthError::InvalidIdToken)?;

        let preferred_username = claims
            .preferred_username()
            .map(|username| username.to_string());

        let email = claims.email().map(|email| email.to_string());

        Ok(OidcClaimsResponseDto {
            sub: claims.subject().to_string(),
            preferred_username: preferred_username,
            email: email,
        })
    }
}

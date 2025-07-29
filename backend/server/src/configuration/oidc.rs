use std::env;

use openidconnect::{IssuerUrl, RedirectUrl, core::CoreProviderMetadata, reqwest};

#[derive(Clone)]
pub struct OidcConfiguration {
    pub provider_metadata: CoreProviderMetadata,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: RedirectUrl,
}

impl OidcConfiguration {
    pub async fn new(http_client: reqwest::Client) -> Self {
        let issuer_url = env::var("OIDC_ISSUER_URL").expect("OIDC_ISSUER_URL must be set");
        let client_id = env::var("OIDC_CLIENT_ID").expect("OIDC_CLIENT_ID must be set");
        let client_secret = env::var("OIDC_CLIENT_SECRET").expect("OIDC_CLIENT_SECRET must be set");
        let redirect_url_str = env::var("OIDC_REDIRECT_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:8080/auth/callback".to_string());

        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new(issuer_url).expect("Invalid issuer URL"),
            &http_client,
        )
        .await
        .expect("Failed to discover OIDC provider metadata");

        Self {
            provider_metadata,
            client_id,
            client_secret,
            redirect_url: RedirectUrl::new(redirect_url_str).expect("Invalid redirect URI"),
        }
    }
}

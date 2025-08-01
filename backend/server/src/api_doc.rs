use actix_web::web;
use auth::framework::web::auth_api_doc;
use image::framework::web::image_api_doc;
use post::framework::web::post_api_doc;
use utoipa::{
    OpenApi,
    openapi::{
        Components, InfoBuilder, OpenApiBuilder,
        security::{AuthorizationCode, Flow, OAuth2, Scopes, SecurityScheme},
    },
};
use utoipa_redoc::{Redoc, Servable};

pub struct ApiDoc;

impl utoipa::OpenApi for ApiDoc {
    fn openapi() -> utoipa::openapi::OpenApi {
        let mut components = Components::new();

        components.add_security_scheme(
            "oauth2",
            SecurityScheme::OAuth2(OAuth2::new(vec![Flow::AuthorizationCode(
                AuthorizationCode::new("/auth/login", "/auth/callback", Scopes::new()),
            )])),
        );

        OpenApiBuilder::new()
            .info(
                InfoBuilder::new()
                    .title("SquidSpirit API")
                    .version(env!("CARGO_PKG_VERSION"))
                    .build(),
            )
            .components(Some(components))
            .build()
    }
}

pub fn configure_api_doc_routes(cfg: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi()
        .merge_from(auth_api_doc::openapi())
        .merge_from(image_api_doc::openapi())
        .merge_from(post_api_doc::openapi());

    cfg.service(Redoc::with_url("/redoc", openapi));
}

use actix_session::Session;
use actix_web::{HttpResponse, Responder, http::header, web};
use anyhow::anyhow;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::{
        auth_controller::AuthController, oidc_callback_query_dto::OidcCallbackQueryDto,
    },
    application::error::auth_error::AuthError,
    framework::web::constants::{
        SESSION_KEY_AUTH_NONCE, SESSION_KEY_AUTH_STATE, SESSION_KEY_USER_ID,
    },
};

#[utoipa::path(
    get,
    path = "/auth/callback",
    tag = "auth",
    summary = "Handle OIDC callback",
    params(
        OidcCallbackQueryDto
    ),
    responses(
        (status = 302, description = "Redirect to home page"),
        (status = 400, description = "Invalid state or nonce"),
    )
)]
pub async fn oidc_callback_handler(
    auth_controller: web::Data<dyn AuthController>,
    query: web::Query<OidcCallbackQueryDto>,
    session: Session,
) -> impl Responder {
    let expected_state = match session.get::<String>(SESSION_KEY_AUTH_STATE) {
        Ok(Some(state)) => state,
        _ => return HttpResponse::BadRequest().finish(),
    };

    let expected_nonce = match session.get::<String>(SESSION_KEY_AUTH_NONCE) {
        Ok(Some(nonce)) => nonce,
        _ => return HttpResponse::BadRequest().finish(),
    };

    let result = auth_controller
        .oidc_callback(query.into_inner(), &expected_state, &expected_nonce)
        .await;

    session.remove(SESSION_KEY_AUTH_STATE);
    session.remove(SESSION_KEY_AUTH_NONCE);
    match result {
        Ok(user) => {
            if let Err(e) = session.insert::<i32>(SESSION_KEY_USER_ID, user.id) {
                capture_anyhow(&e.into());
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Found()
                .append_header((header::LOCATION, "/"))
                .finish()
        }
        Err(e) => match e {
            AuthError::InvalidAuthCode
            | AuthError::InvalidIdToken
            | AuthError::InvalidNonce
            | AuthError::InvalidState => HttpResponse::BadRequest().finish(),
            _ => {
                match e {
                    AuthError::Unexpected(e) => capture_anyhow(&e),
                    _ => capture_anyhow(&anyhow!(e)),
                };
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

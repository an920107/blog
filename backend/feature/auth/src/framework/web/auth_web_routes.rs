use actix_session::Session;
use actix_web::{HttpResponse, Responder, http::header, web};

use crate::{
    adapter::delivery::{
        auth_controller::AuthController, oidc_callback_query_dto::OidcCallbackQueryDto,
    },
    application::error::auth_error::AuthError,
    framework::web::constants::{
        SESSION_KEY_AUTH_NONCE, SESSION_KEY_AUTH_STATE, SESSION_KEY_USER_ID,
    },
};

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::get().to(oidc_login_handler))
            .route("/callback", web::get().to(oidc_callback_handler))
            .route("/logout", web::get().to(logout_handler)),
    );
}

async fn oidc_login_handler(
    auth_controller: web::Data<dyn AuthController>,
    session: Session,
) -> impl Responder {
    let result = auth_controller.oidc_login();

    match result {
        Ok(auth_url) => {
            if let Err(e) = session.insert::<String>(SESSION_KEY_AUTH_STATE, auth_url.state) {
                log::error!("{e:?}");
                return HttpResponse::InternalServerError().finish();
            }
            if let Err(e) = session.insert::<String>(SESSION_KEY_AUTH_NONCE, auth_url.nonce) {
                log::error!("{e:?}");
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Found()
                .append_header((header::LOCATION, auth_url.url))
                .finish()
        }
        Err(e) => {
            log::error!("{e:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn oidc_callback_handler(
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
                log::error!("{e:?}");
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
                log::error!("{e:?}");
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

async fn logout_handler(session: Session) -> impl Responder {
    session.remove(SESSION_KEY_AUTH_STATE);
    session.remove(SESSION_KEY_AUTH_NONCE);
    session.remove(SESSION_KEY_USER_ID);
    HttpResponse::Found()
        .append_header((header::LOCATION, "/"))
        .finish()
}

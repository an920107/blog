use actix_session::Session;
use actix_web::{HttpResponse, Responder, http::header, web};

use crate::{
    adapter::delivery::auth_controller::AuthController,
    framework::web::constants::{SESSION_KEY_AUTH_NONCE, SESSION_KEY_AUTH_STATE},
};

#[utoipa::path(
    get,
    path = "/auth/login",
    tag = "auth",
    summary = "Initiate OIDC login",
    responses(
        (status = 302, description = "Redirect to OIDC provider")
    )
)]
pub async fn oidc_login_handler(
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

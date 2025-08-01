use actix_session::Session;
use actix_web::{HttpResponse, Responder, http::header};

#[utoipa::path(
    get,
    path = "/auth/logout",
    tag = "auth",
    summary = "Logout user",
    responses(
        (status = 302, description = "Redirect to home page")
    )
)]
pub async fn oidc_logout_handler(session: Session) -> impl Responder {
    session.clear();
    HttpResponse::Found()
        .append_header((header::LOCATION, "/"))
        .finish()
}

use actix_web::web;

use crate::framework::web::{
    get_logged_in_user_handler::get_logged_in_user_handler,
    oidc_callback_handler::oidc_callback_handler, oidc_login_handler::oidc_login_handler,
    oidc_logout_handler::oidc_logout_handler,
};

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::get().to(oidc_login_handler))
            .route("/callback", web::get().to(oidc_callback_handler))
            .route("/logout", web::get().to(oidc_logout_handler)),
    );

    cfg.service(web::resource("/me").route(web::get().to(get_logged_in_user_handler)));
}

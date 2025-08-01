use crate::framework::web::{
    get_logged_in_user_handler, oidc_callback_handler, oidc_login_handler, oidc_logout_handler,
};
use utoipa::{OpenApi, openapi};

#[derive(OpenApi)]
#[openapi(paths(
    get_logged_in_user_handler::get_logged_in_user_handler,
    oidc_callback_handler::oidc_callback_handler,
    oidc_login_handler::oidc_login_handler,
    oidc_logout_handler::oidc_logout_handler
))]
struct ApiDoc;

pub fn openapi() -> openapi::OpenApi {
    ApiDoc::openapi()
}

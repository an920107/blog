use actix_web::{HttpResponse, Responder, web};
use anyhow::anyhow;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::{auth_controller::AuthController, user_response_dto::UserResponseDto},
    application::error::auth_error::AuthError,
    framework::web::auth_middleware::UserId,
};

#[utoipa::path(
    get,
    path = "/me",
    tag = "auth",
    summary = "Get logged-in user information",
    responses(
        (status = 200, body = UserResponseDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
pub async fn get_logged_in_user_handler(
    auth_controller: web::Data<dyn AuthController>,
    user_id: UserId,
) -> impl Responder {
    let result = auth_controller.get_user(user_id.get()).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            match e {
                AuthError::Unexpected(e) => capture_anyhow(&e),
                _ => capture_anyhow(&anyhow!(e)),
            };
            HttpResponse::InternalServerError().finish()
        }
    }
}

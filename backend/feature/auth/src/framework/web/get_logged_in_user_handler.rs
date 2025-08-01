use actix_web::{HttpResponse, Responder, web};

use crate::{
    adapter::delivery::{auth_controller::AuthController, user_response_dto::UserResponseDto},
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
            log::error!("{e:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

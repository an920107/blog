use actix_web::{HttpResponse, Responder, web};
use anyhow::anyhow;
use auth::framework::web::auth_middleware::UserId;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::{
        create_post_request_dto::CreatePostRequestDto, post_controller::PostController,
        post_response_dto::PostResponseDto,
    },
    application::error::post_error::PostError,
};

#[utoipa::path(
    post,
    path = "/post",
    tag = "post",
    summary = "Create a new post",
    responses(
        (status = 201, body = PostResponseDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
pub async fn create_post_handler(
    post_controller: web::Data<dyn PostController>,
    post_dto: web::Json<CreatePostRequestDto>,
    user_id: UserId,
) -> impl Responder {
    let result = post_controller
        .create_post(post_dto.into_inner(), user_id.get())
        .await;

    match result {
        Ok(post) => HttpResponse::Created().json(post),
        Err(e) => {
            match e {
                PostError::Unexpected(e) => capture_anyhow(&e),
                _ => capture_anyhow(&anyhow!(e)),
            };
            HttpResponse::InternalServerError().finish()
        }
    }
}

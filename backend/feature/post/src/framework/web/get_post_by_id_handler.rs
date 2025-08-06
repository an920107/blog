use actix_web::{HttpResponse, Responder, web};
use auth::framework::web::auth_middleware::UserId;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::{post_controller::PostController, post_response_dto::PostResponseDto},
    application::error::post_error::PostError,
};

#[utoipa::path(
    get,
    path = "/post/{id}",
    tag = "post",
    summary = "Get post by ID",
    description = "Only authenticated users can access unpublished posts.",
    responses (
        (status = 200, body = PostResponseDto),
        (status = 404, description = "Post not found")
    )
)]
pub async fn get_post_by_id_handler(
    post_controller: web::Data<dyn PostController>,
    path: web::Path<i32>,
    user_id: Option<UserId>,
) -> impl Responder {
    let id = path.into_inner();
    let result = post_controller
        .get_post_by_id(id, user_id.map(|id| id.get()))
        .await;

    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => match e {
            PostError::NotFound => HttpResponse::NotFound().finish(),
            PostError::Unauthorized => HttpResponse::Unauthorized().finish(),
            PostError::Unexpected(e) => {
                capture_anyhow(&e);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

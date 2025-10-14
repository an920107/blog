use actix_web::{HttpResponse, Responder, web};
use auth::framework::web::auth_middleware::UserId;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::{
        post_controller::PostController, post_response_dto::PostResponseDto,
        update_post_request_dto::UpdatePostRequestDto,
    },
    application::error::post_error::PostError,
};

#[utoipa::path(
    put,
    path = "/post/{id}",
    tag = "post",
    summary = "Update a post by ID",
    responses(
        (status = 200, body = PostResponseDto),
        (status = 400, description = PostError::InvalidSemanticId),
        (status = 401, description = PostError::Unauthorized),
        (status = 404, description = PostError::NotFound),
        (status = 409, description = PostError::DuplicatedSemanticId),
    ),
    security(
        ("oauth2" = [])
    )
)]
pub async fn update_post_handler(
    post_controller: web::Data<dyn PostController>,
    path: web::Path<i32>,
    post_dto: web::Json<UpdatePostRequestDto>,
    user_id: UserId,
) -> impl Responder {
    let id = path.into_inner();
    let result = post_controller
        .update_post(id, post_dto.into_inner(), user_id.get())
        .await;

    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => match e {
            PostError::NotFound => HttpResponse::NotFound().finish(),
            PostError::Unauthorized => HttpResponse::Unauthorized().finish(),
            PostError::DuplicatedSemanticId => HttpResponse::Conflict().finish(),
            PostError::InvalidSemanticId => HttpResponse::BadRequest().finish(),
            PostError::Unexpected(e) => {
                capture_anyhow(&e);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

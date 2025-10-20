use actix_web::{HttpResponse, Responder, web};
use anyhow::anyhow;
use auth::framework::web::auth_middleware::UserId;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::{post_controller::PostController, post_response_dto::PostResponseDto},
    domain::error::post_error::PostError,
};

#[utoipa::path(
    get,
    path = "/post/{id}",
    tag = "post",
    summary = "Get post by ID or semantic ID",
    description = "Only authenticated users can access unpublished posts. Accepts either numeric ID or semantic ID.",
    responses (
        (status = 200, body = PostResponseDto),
        (status = 404, description = PostError::NotFound),
    )
)]
pub async fn get_post_by_id_handler(
    post_controller: web::Data<dyn PostController>,
    path: web::Path<String>,
    user_id: Option<UserId>,
) -> impl Responder {
    let id_or_semantic_id = path.into_inner();
    let result = post_controller
        .get_post_by_id_or_semantic_id(&id_or_semantic_id, user_id.map(|id| id.get()))
        .await;

    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => match e {
            PostError::NotFound => HttpResponse::NotFound().finish(),
            PostError::InvalidSemanticId
            | PostError::DuplicatedSemanticId
            | PostError::LabelNotFound => {
                capture_anyhow(&anyhow!(e));
                HttpResponse::InternalServerError().finish()
            }
            PostError::Unexpected(e) => {
                capture_anyhow(&e);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

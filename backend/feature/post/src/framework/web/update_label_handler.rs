use actix_web::{HttpResponse, Responder, web};
use anyhow::anyhow;
use auth::framework::web::auth_middleware::UserId;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::{
        label_response_dto::LabelResponseDto, post_controller::PostController,
        update_label_request_dto::UpdateLabelRequestDto,
    },
    application::error::post_error::PostError,
};

#[utoipa::path(
    put,
    path = "/label/{id}",
    tag = "post",
    summary = "Update a label by ID",
    responses(
        (status = 200, body = LabelResponseDto),
        (status = 404, description = "Label not found"),
    ),
    security(
        ("oauth2" = [])
    )
)]
pub async fn update_label_handler(
    post_controller: web::Data<dyn PostController>,
    label_dto: web::Json<UpdateLabelRequestDto>,
    path: web::Path<i32>,
    _: UserId,
) -> impl Responder {
    let id = path.into_inner();
    let result = post_controller
        .update_label(id, label_dto.into_inner())
        .await;

    match result {
        Ok(label) => HttpResponse::Ok().json(label),
        Err(e) => match e {
            PostError::NotFound => HttpResponse::NotFound().finish(),
            PostError::Unauthorized => HttpResponse::Unauthorized().finish(),
            PostError::DuplicatedLabelName => HttpResponse::Conflict().finish(),
            PostError::InvalidSemanticId | PostError::DuplicatedSemanticId => {
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

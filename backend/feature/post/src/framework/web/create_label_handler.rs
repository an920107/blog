use actix_web::{HttpResponse, Responder, web};
use anyhow::anyhow;
use auth::framework::web::auth_middleware::UserId;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::{
        create_label_request_dto::CreateLabelRequestDto, label_response_dto::LabelResponseDto,
        post_controller::PostController,
    },
    application::error::post_error::PostError,
};

#[utoipa::path(
    post,
    path = "/label",
    tag = "post",
    summary = "Create a new label",
    responses(
        (status = 201, body = LabelResponseDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
pub async fn create_label_handler(
    post_controller: web::Data<dyn PostController>,
    label_dto: web::Json<CreateLabelRequestDto>,
    _: UserId,
) -> impl Responder {
    let result = post_controller.create_label(label_dto.into_inner()).await;

    match result {
        Ok(label) => HttpResponse::Created().json(label),
        Err(e) => match e {
            PostError::Unauthorized => HttpResponse::Unauthorized().finish(),
            PostError::NotFound | PostError::InvalidSemanticId => {
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

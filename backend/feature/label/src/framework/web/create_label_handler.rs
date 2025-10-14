use actix_web::{HttpResponse, Responder, web};
use anyhow::anyhow;
use auth::framework::web::auth_middleware::UserId;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::{
        create_label_request_dto::CreateLabelRequestDto, label_controller::LabelController,
        label_response_dto::LabelResponseDto,
    },
    application::error::label_error::LabelError,
};

#[utoipa::path(
    post,
    path = "/label",
    tag = "label",
    summary = "Create a new label",
    responses(
        (status = 201, body = LabelResponseDto),
        (status = 401, description = LabelError::Unauthorized),
        (status = 409, description = LabelError::DuplicatedLabelName),
    ),
    security(
        ("oauth2" = [])
    )
)]
pub async fn create_label_handler(
    label_controller: web::Data<dyn LabelController>,
    label_dto: web::Json<CreateLabelRequestDto>,
    _: UserId,
) -> impl Responder {
    let result = label_controller.create_label(label_dto.into_inner()).await;

    match result {
        Ok(label) => HttpResponse::Created().json(label),
        Err(e) => match e {
            LabelError::Unauthorized => HttpResponse::Unauthorized().finish(),
            LabelError::DuplicatedLabelName => HttpResponse::Conflict().finish(),
            LabelError::NotFound => {
                capture_anyhow(&anyhow!(e));
                HttpResponse::InternalServerError().finish()
            }
            LabelError::Unexpected(e) => {
                capture_anyhow(&e);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

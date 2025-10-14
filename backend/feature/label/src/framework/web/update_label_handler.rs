use actix_web::{HttpResponse, Responder, web};
use auth::framework::web::auth_middleware::UserId;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::{
        label_controller::LabelController, label_response_dto::LabelResponseDto,
        update_label_request_dto::UpdateLabelRequestDto,
    },
    application::error::label_error::LabelError,
};

#[utoipa::path(
    put,
    path = "/label/{id}",
    tag = "label",
    summary = "Update a label by ID",
    responses(
        (status = 200, body = LabelResponseDto),
        (status = 401, description = LabelError::Unauthorized),
        (status = 404, description = LabelError::NotFound),
        (status = 409, description = LabelError::DuplicatedLabelName),
    ),
    security(
        ("oauth2" = [])
    )
)]
pub async fn update_label_handler(
    label_controller: web::Data<dyn LabelController>,
    label_dto: web::Json<UpdateLabelRequestDto>,
    path: web::Path<i32>,
    _: UserId,
) -> impl Responder {
    let id = path.into_inner();
    let result = label_controller
        .update_label(id, label_dto.into_inner())
        .await;

    match result {
        Ok(label) => HttpResponse::Ok().json(label),
        Err(e) => match e {
            LabelError::NotFound => HttpResponse::NotFound().finish(),
            LabelError::Unauthorized => HttpResponse::Unauthorized().finish(),
            LabelError::DuplicatedLabelName => HttpResponse::Conflict().finish(),
            LabelError::Unexpected(e) => {
                capture_anyhow(&e);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

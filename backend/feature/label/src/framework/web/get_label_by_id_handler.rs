use actix_web::{HttpResponse, Responder, web};
use anyhow::anyhow;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::{label_controller::LabelController, label_response_dto::LabelResponseDto},
    application::error::label_error::LabelError,
};

#[utoipa::path(
    get,
    path = "/label/{id}",
    tag = "label",
    summary = "Get the label by ID",
    responses(
        (status = 200, body = LabelResponseDto),
        (status = 404, description = LabelError::NotFound),
    )
)]
pub async fn get_label_by_id_handler(
    label_controller: web::Data<dyn LabelController>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    let result = label_controller.get_label_by_id(id).await;

    match result {
        Ok(label) => HttpResponse::Ok().json(label),
        Err(e) => match e {
            LabelError::NotFound => HttpResponse::NotFound().finish(),
            LabelError::Unauthorized | LabelError::DuplicatedLabelName => {
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

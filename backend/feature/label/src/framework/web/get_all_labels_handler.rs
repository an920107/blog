use actix_web::{HttpResponse, Responder, web};
use anyhow::anyhow;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::{label_controller::LabelController, label_response_dto::LabelResponseDto},
    domain::error::label_error::LabelError,
};

#[utoipa::path(
    get,
    path = "/label",
    tag = "label",
    summary = "Get all labels",
    responses(
        (status = 200, body = Vec<LabelResponseDto>)
    )
)]
pub async fn get_all_labels_handler(
    label_controller: web::Data<dyn LabelController>,
) -> impl Responder {
    let result = label_controller.get_all_labels().await;

    match result {
        Ok(labels) => HttpResponse::Ok().json(labels),
        Err(e) => match e {
            LabelError::NotFound | LabelError::DuplicatedLabelName => {
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

use actix_web::{HttpResponse, Responder, web};
use anyhow::anyhow;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::{label_response_dto::LabelResponseDto, post_controller::PostController},
    application::error::post_error::PostError,
};

#[utoipa::path(
    get,
    path = "/label",
    tag = "post",
    summary = "Get all labels",
    responses(
        (status = 200, body = Vec<LabelResponseDto>)
    )
)]
pub async fn get_all_labels_handler(
    post_controller: web::Data<dyn PostController>,
) -> impl Responder {
    let result = post_controller.get_all_labels().await;

    match result {
        Ok(labels) => HttpResponse::Ok().json(labels),
        Err(e) => {
            match e {
                PostError::Unexpected(e) => capture_anyhow(&e),
                _ => capture_anyhow(&anyhow!(e)),
            };
            HttpResponse::InternalServerError().finish()
        }
    }
}

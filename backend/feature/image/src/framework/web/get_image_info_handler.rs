use actix_web::{HttpResponse, Responder, web};
use anyhow::anyhow;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::{
        image_controller::ImageController, image_info_response_dto::ImageInfoResponseDto,
    },
    domain::error::image_error::ImageError,
};

#[utoipa::path(
    get,
    path = "/image/{id}/info",
    tag = "image",
    summary = "Get image info by ID",
    responses (
        (status = 200, body = ImageInfoResponseDto),
        (status = 404, description = ImageError::NotFound),
    )
)]
pub async fn get_image_info_handler(
    image_controller: web::Data<dyn ImageController>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    let result = image_controller.get_image_info(id).await;

    match result {
        Ok(image_info) => HttpResponse::Ok().json(image_info),
        Err(e) => match e {
            ImageError::NotFound => HttpResponse::NotFound().finish(),
            ImageError::UnsupportedMimeType(_) => {
                capture_anyhow(&anyhow!(e));
                HttpResponse::InternalServerError().finish()
            }
            ImageError::Unexpected(e) => {
                capture_anyhow(&e);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

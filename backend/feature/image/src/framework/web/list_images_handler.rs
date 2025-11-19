use actix_web::{HttpResponse, Responder, web};
use auth::framework::web::auth_middleware::UserId;
use sentry::integrations::anyhow::capture_anyhow;
use anyhow::anyhow;

use crate::{
    adapter::delivery::{
        image_controller::ImageController, image_info_response_dto::ImageInfoResponseDto,
    },
    domain::error::image_error::ImageError,
};

#[utoipa::path(
    get,
    path = "/image",
    tag = "image",
    summary = "List all images",
    responses (
        (status = 200, body = [ImageInfoResponseDto])
    )
)]
pub async fn list_images_handler(
    image_controller: web::Data<dyn ImageController>,
    _: UserId,
) -> impl Responder {
    let result = image_controller.list_images().await;

    match result {
        Ok(images) => HttpResponse::Ok().json(images),
        Err(e) => match e {
            ImageError::NotFound => {
                capture_anyhow(&anyhow!(e));
                HttpResponse::InternalServerError().finish()
            }
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

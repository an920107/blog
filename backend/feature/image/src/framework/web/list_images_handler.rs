use actix_web::{HttpResponse, Responder, web};
use anyhow::anyhow;
use auth::framework::web::auth_middleware::UserId;
use sentry::integrations::anyhow::capture_anyhow;

use crate::adapter::delivery::{
    image_controller::ImageController, image_info_response_dto::ImageInfoResponseDto,
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
        Err(e) => {
            capture_anyhow(&anyhow!(e));
            HttpResponse::InternalServerError().finish()
        }
    }
}

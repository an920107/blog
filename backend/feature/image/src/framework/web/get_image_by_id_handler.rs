use actix_web::{HttpResponse, Responder, web};
use anyhow::anyhow;
use sentry::integrations::anyhow::capture_anyhow;
use utoipa::ToSchema;

use crate::{
    adapter::delivery::image_controller::ImageController,
    application::error::image_error::ImageError,
};

#[utoipa::path(
    get,
    path = "/image/{id}",
    tag = "image",
    summary = "Get image by ID",
    responses (
        (status = 200, body = inline(ResponseBodySchema), content_type = "image/*"),
        (status = 404, description = ImageError::NotFound),
    )
)]
pub async fn get_image_by_id_handler(
    image_controller: web::Data<dyn ImageController>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    let result = image_controller.get_image_by_id(id).await;

    match result {
        Ok(image_response) => HttpResponse::Ok()
            .content_type(image_response.mime_type)
            .body(image_response.data),
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

#[derive(ToSchema)]
#[schema(value_type = String, format = Binary)]
#[allow(dead_code)]
struct ResponseBodySchema(Vec<u8>);

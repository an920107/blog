use actix_web::{HttpResponse, Responder, web};
use anyhow::anyhow;
use auth::framework::web::auth_middleware::UserId;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::image_controller::ImageController, domain::error::image_error::ImageError,
};

#[utoipa::path(
    delete,
    path = "/image/{id}",
    tag = "image",
    summary = "Delete an image by ID",
    responses (
        (status = 204, description = "Image deleted successfully"),
        (status = 400, description = "Image is referenced by one or more posts and cannot be deleted"),
        (status = 404, description = "Image not found"),
    ),
    security(
        ("oauth2" = [])
    )
)]
pub async fn delete_image_handler(
    image_controller: web::Data<dyn ImageController>,
    path: web::Path<i32>,
    _: UserId,
) -> impl Responder {
    let id = path.into_inner();
    let result = image_controller.delete_image(id).await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => match e {
            ImageError::NotFound => HttpResponse::NotFound().finish(),
            ImageError::ReferencedImage => HttpResponse::BadRequest()
                .body("Image is referenced by one or more posts and cannot be deleted"),
            _ => {
                capture_anyhow(&anyhow!(e));
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

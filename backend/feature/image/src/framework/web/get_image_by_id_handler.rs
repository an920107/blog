use actix_web::{
    HttpRequest, HttpResponse, Responder,
    http::header::{CACHE_CONTROL, ETAG, IF_NONE_MATCH},
    web,
};
use anyhow::anyhow;
use fnv_rs::{Fnv64, FnvHasher};
use sentry::integrations::anyhow::capture_anyhow;
use utoipa::ToSchema;

use crate::{
    adapter::delivery::image_controller::ImageController, domain::error::image_error::ImageError,
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
    request: HttpRequest,
    image_controller: web::Data<dyn ImageController>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    let result = image_controller.get_image_by_id(id).await;

    match result {
        Ok(image_response) => {
            let etag_value = generate_etag(image_response.id, &image_response.data);

            let etag_header = (ETAG, etag_value.clone());
            let cache_control_header = (CACHE_CONTROL, "public, max-age=31536000, immutable");

            if is_not_modified(&request, &etag_value) {
                HttpResponse::NotModified()
                    .append_header(etag_header)
                    .append_header(cache_control_header)
                    .finish()
            } else {
                HttpResponse::Ok()
                    .append_header(etag_header)
                    .append_header(cache_control_header)
                    .content_type(image_response.mime_type)
                    .body(image_response.data)
            }
        }
        Err(e) => match e {
            ImageError::NotFound => HttpResponse::NotFound().finish(),
            _ => {
                capture_anyhow(&anyhow!(e));
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

#[derive(ToSchema)]
#[schema(value_type = String, format = Binary)]
#[allow(dead_code)]
struct ResponseBodySchema(Vec<u8>);

fn is_not_modified(request: &HttpRequest, etag_value: &str) -> bool {
    request
        .headers()
        .get(IF_NONE_MATCH)
        .and_then(|value| value.to_str().ok())
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .any(|candidate| candidate == "*" || candidate == etag_value)
        })
        .unwrap_or(false)
}

fn generate_etag(image_id: i32, data: &[u8]) -> String {
    let hash = Fnv64::hash(data);
    format!("\"img-{image_id}-{hash:016x}-{:x}\"", data.len())
}

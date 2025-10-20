use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder, web};
use anyhow::anyhow;
use auth::framework::web::auth_middleware::UserId;
use futures::StreamExt;
use sentry::integrations::anyhow::capture_anyhow;
use utoipa::ToSchema;

use crate::{
    adapter::delivery::{
        image_controller::ImageController, image_info_response_dto::ImageInfoResponseDto,
        image_request_dto::ImageRequestDto,
    },
    domain::error::image_error::ImageError,
};

#[utoipa::path(
    post,
    path = "/image/upload",
    tag = "image",
    summary = "Upload an image",
    request_body (
        content = RequestBodySchema,
        content_type = "multipart/form-data",
    ),
    responses (
        (status = 201, body = ImageInfoResponseDto),
        (status = 400, description = ImageError::UnsupportedMimeType("{MIME_TYPE}".to_string())),
    ),
    security(
        ("oauth2" = [])
    )
)]
pub async fn upload_image_handler(
    image_controller: web::Data<dyn ImageController>,
    mut payload: Multipart,
    _: UserId,
) -> impl Responder {
    let mut image_request_dto: Option<ImageRequestDto> = None;

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(_) => return HttpResponse::BadRequest().finish(),
        };

        if field.name() != Some("file") {
            continue;
        }

        let mime_type = field
            .content_type()
            .cloned()
            .map(|mt| mt.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let mut data = Vec::new();
        while let Some(chunk) = field.next().await {
            match chunk {
                Ok(bytes) => data.extend_from_slice(&bytes),
                Err(_) => return HttpResponse::InternalServerError().finish(),
            }
        }

        image_request_dto = Some(ImageRequestDto { mime_type, data });
        break;
    }

    let image_request_dto = match image_request_dto {
        Some(dto) => dto,
        None => return HttpResponse::BadRequest().finish(),
    };
    let result = image_controller.upload_image(image_request_dto).await;

    match result {
        Ok(image_info) => HttpResponse::Created().json(image_info),
        Err(e) => match e {
            ImageError::UnsupportedMimeType(mime_type) => {
                HttpResponse::BadRequest().body(format!("Unsupported MIME type: {}", mime_type))
            }
            ImageError::NotFound => {
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
#[allow(dead_code)]
struct RequestBodySchema {
    #[schema(value_type = String, format = Binary)]
    file: Vec<u8>,
}

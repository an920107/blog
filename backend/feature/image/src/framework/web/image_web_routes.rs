use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder, web};
use auth::framework::web::auth_middleware::UserId;
use futures::StreamExt;

use crate::{
    adapter::delivery::{image_controller::ImageController, image_request_dto::ImageRequestDto},
    application::error::image_error::ImageError,
};

pub fn configure_image_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/image")
            .route("/upload", web::post().to(upload_image_handler))
            .route("/{id}", web::get().to(get_image_by_id_handler)),
    );
}

async fn upload_image_handler(
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
            ImageError::UnsupportedMimeType => HttpResponse::BadRequest().body(format!("{e:?}")),
            _ => {
                log::error!("{e:?}");
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

async fn get_image_by_id_handler(
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
            _ => {
                log::error!("{e:?}");
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

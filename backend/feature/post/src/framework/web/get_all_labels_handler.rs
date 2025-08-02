use actix_web::{HttpResponse, Responder, web};

use crate::adapter::delivery::{
    label_response_dto::LabelResponseDto, post_controller::PostController,
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
            log::error!("{e:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

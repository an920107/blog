use actix_web::{HttpResponse, Responder, web};
use auth::framework::web::auth_middleware::UserId;

use crate::adapter::delivery::{
    create_label_request_dto::CreateLabelRequestDto, label_response_dto::LabelResponseDto,
    post_controller::PostController,
};

#[utoipa::path(
    post,
    path = "/label",
    tag = "post",
    summary = "Create a new label",
    responses(
        (status = 201, body = LabelResponseDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
pub async fn create_label_handler(
    post_controller: web::Data<dyn PostController>,
    label_dto: web::Json<CreateLabelRequestDto>,
    _: UserId,
) -> impl Responder {
    let result = post_controller.create_label(label_dto.into_inner()).await;

    match result {
        Ok(label) => HttpResponse::Created().json(label),
        Err(e) => {
            log::error!("{e:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

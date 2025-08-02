use actix_web::{web, HttpResponse, Responder};
use auth::framework::web::auth_middleware::UserId;

use crate::adapter::delivery::{
    create_post_request_dto::CreatePostRequestDto, post_controller::PostController,
    post_response_dto::PostResponseDto,
};

#[utoipa::path(
    post,
    path = "/post",
    tag = "post",
    summary = "Create a new post",
    responses(
        (status = 201, body = PostResponseDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
pub async fn create_post_handler(
    post_controller: web::Data<dyn PostController>,
    post_dto: web::Json<CreatePostRequestDto>,
    _: UserId,
) -> impl Responder {
    let result = post_controller.create_post(post_dto.into_inner()).await;

    match result {
        Ok(post) => HttpResponse::Created().json(post),
        Err(e) => {
            log::error!("{e:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

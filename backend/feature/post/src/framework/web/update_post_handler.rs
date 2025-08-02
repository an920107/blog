use actix_web::{HttpResponse, Responder, web};
use auth::framework::web::auth_middleware::UserId;

use crate::adapter::delivery::{
    post_controller::PostController, post_response_dto::PostResponseDto,
    update_post_request_dto::UpdatePostRequestDto,
};

#[utoipa::path(
    put,
    path = "/post/{id}",
    tag = "post",
    summary = "Update a post by ID",
    responses(
        (status = 200, body = PostResponseDto),
        (status = 404, description = "Post not found"),
    ),
    security(
        ("oauth2" = [])
    )
)]
pub async fn update_post_handler(
    post_controller: web::Data<dyn PostController>,
    path: web::Path<i32>,
    post_dto: web::Json<UpdatePostRequestDto>,
    _: UserId,
) -> impl Responder {
    let id = path.into_inner();
    let result = post_controller.update_post(id, post_dto.into_inner()).await;

    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => {
            log::error!("{e:?}");
            match e {
                crate::application::error::post_error::PostError::NotFound => {
                    HttpResponse::NotFound().finish()
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

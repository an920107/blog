use actix_web::{HttpResponse, Responder, web};

use crate::{
    adapter::delivery::{post_controller::PostController, post_response_dto::PostResponseDto},
    application::error::post_error::PostError,
};

#[utoipa::path(
    get,
    path = "/post/{id}",
    tag = "post",
    summary = "Get post by ID",
    responses (
        (status = 200, body = PostResponseDto),
        (status = 404, description = "Post not found")
    )
)]
pub async fn get_post_by_id_handler(
    post_controller: web::Data<dyn PostController>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    let result = post_controller.get_post_by_id(id).await;

    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => {
            if e == PostError::NotFound {
                HttpResponse::NotFound().finish()
            } else {
                log::error!("{e:?}");
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

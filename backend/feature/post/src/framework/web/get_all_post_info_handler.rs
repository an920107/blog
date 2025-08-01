use actix_web::{HttpResponse, Responder, web};

use crate::adapter::delivery::{
    post_controller::PostController, post_info_query_dto::PostQueryDto,
    post_info_response_dto::PostInfoResponseDto,
};

#[utoipa::path(
    get,
    path = "/post/all",
    tag = "post",
    summary = "Get all post information",
    params(
        PostQueryDto
    ),
    responses (
        (status = 200, body = [PostInfoResponseDto])
    )
)]
pub async fn get_all_post_info_handler(
    post_controller: web::Data<dyn PostController>,
    query: web::Query<PostQueryDto>,
) -> impl Responder {
    let result = post_controller.get_all_post_info(query.into_inner()).await;

    match result {
        Ok(post_info_list) => HttpResponse::Ok().json(post_info_list),
        Err(e) => {
            log::error!("{e:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

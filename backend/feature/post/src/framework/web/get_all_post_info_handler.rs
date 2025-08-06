use actix_web::{HttpResponse, Responder, web};
use anyhow::anyhow;
use auth::framework::web::auth_middleware::UserId;
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    adapter::delivery::{
        post_controller::PostController, post_info_query_dto::PostQueryDto,
        post_info_response_dto::PostInfoResponseDto,
    },
    application::error::post_error::PostError,
};

#[utoipa::path(
    get,
    path = "/post",
    tag = "post",
    summary = "Get all post information",
    description = "`is_published_only` query is only available for authenticated users.",
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
    user_id: Option<UserId>,
) -> impl Responder {
    let result = post_controller
        .get_all_post_info(query.into_inner(), user_id.map(|id| id.get()))
        .await;

    match result {
        Ok(post_info_list) => HttpResponse::Ok().json(post_info_list),
        Err(e) => {
            match e {
                PostError::Unexpected(e) => capture_anyhow(&e),
                _ => capture_anyhow(&anyhow!(e)),
            };
            HttpResponse::InternalServerError().finish()
        }
    }
}

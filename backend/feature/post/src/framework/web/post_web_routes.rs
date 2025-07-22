use actix_web::{HttpResponse, Responder, web};

use crate::{
    adapter::delivery::{post_controller::PostController, post_info_query_dto::PostQueryDto},
    application::error::post_error::PostError,
};

pub fn configure_post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .route("/all", web::get().to(get_all_post_info_handler))
            .route("/{id}", web::get().to(get_full_post_handler)),
    );
}

async fn get_all_post_info_handler(
    post_controller: web::Data<dyn PostController>,
    query: web::Query<PostQueryDto>,
) -> impl Responder {
    let is_published_only = query.is_published_only.unwrap_or_else(|| true);
    let result = post_controller.get_all_post_info(is_published_only).await;

    match result {
        Ok(post_info_list) => HttpResponse::Ok().json(post_info_list),
        Err(e) => {
            log::error!("{e:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_full_post_handler(
    post_controller: web::Data<dyn PostController>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    let result = post_controller.get_full_post(id).await;

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

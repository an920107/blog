use actix_web::web;

use crate::framework::web::{
    get_all_post_info_handler::get_all_post_info_handler,
    get_post_by_id_handler::get_post_by_id_handler,
};

pub fn configure_post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .route("/all", web::get().to(get_all_post_info_handler))
            .route("/{id}", web::get().to(get_post_by_id_handler)),
    );
}

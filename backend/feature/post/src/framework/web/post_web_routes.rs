use actix_web::web;

use crate::framework::web::{
    create_label_handler::create_label_handler, create_post_handler::create_post_handler,
    get_all_labels_handler::get_all_labels_handler,
    get_all_post_info_handler::get_all_post_info_handler,
    get_post_by_id_handler::get_post_by_id_handler, update_label_handler::update_label_handler,
    update_post_handler::update_post_handler,
};

pub fn configure_post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .route("", web::get().to(get_all_post_info_handler))
            .route("", web::post().to(create_post_handler))
            .route("/{id}", web::get().to(get_post_by_id_handler))
            .route("/{id}", web::put().to(update_post_handler)),
    );

    cfg.service(
        web::scope("/label")
            .route("", web::get().to(get_all_labels_handler))
            .route("", web::post().to(create_label_handler))
            .route("/{id}", web::put().to(update_label_handler)),
    );
}

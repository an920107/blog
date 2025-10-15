use actix_web::web;

use crate::framework::web::{
    create_label_handler::create_label_handler, get_all_labels_handler::get_all_labels_handler,
    get_label_by_id_handler::get_label_by_id_handler, update_label_handler::update_label_handler,
};

pub fn configure_label_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/label")
            .route("", web::get().to(get_all_labels_handler))
            .route("/{id}", web::get().to(get_label_by_id_handler))
            .route("", web::post().to(create_label_handler))
            .route("/{id}", web::put().to(update_label_handler)),
    );
}

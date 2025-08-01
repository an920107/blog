use actix_web::web;

use crate::framework::web::{
    get_image_by_id_handler::get_image_by_id_handler, upload_image_handler::upload_image_handler,
};

pub fn configure_image_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/image")
            .route("/upload", web::post().to(upload_image_handler))
            .route("/{id}", web::get().to(get_image_by_id_handler)),
    );
}

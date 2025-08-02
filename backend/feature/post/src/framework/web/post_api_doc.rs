use crate::framework::web::{
    create_label_handler, create_post_handler, get_all_labels_handler, get_all_post_info_handler,
    get_post_by_id_handler, update_label_handler, update_post_handler,
};
use utoipa::{OpenApi, openapi};

#[derive(OpenApi)]
#[openapi(paths(
    get_all_post_info_handler::get_all_post_info_handler,
    get_post_by_id_handler::get_post_by_id_handler,
    create_post_handler::create_post_handler,
    update_post_handler::update_post_handler,
    create_label_handler::create_label_handler,
    update_label_handler::update_label_handler,
    get_all_labels_handler::get_all_labels_handler
))]
struct ApiDoc;

pub fn openapi() -> openapi::OpenApi {
    ApiDoc::openapi()
}

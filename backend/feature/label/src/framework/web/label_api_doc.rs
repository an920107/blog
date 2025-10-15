use crate::framework::web::{
    create_label_handler, get_all_labels_handler, get_label_by_id_handler, update_label_handler,
};
use utoipa::{OpenApi, openapi};

#[derive(OpenApi)]
#[openapi(paths(
    create_label_handler::create_label_handler,
    update_label_handler::update_label_handler,
    get_all_labels_handler::get_all_labels_handler,
    get_label_by_id_handler::get_label_by_id_handler
))]
struct ApiDoc;

pub fn openapi() -> openapi::OpenApi {
    ApiDoc::openapi()
}

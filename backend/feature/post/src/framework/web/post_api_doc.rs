use crate::framework::web::{get_all_post_info_handler, get_post_by_id_handler};
use utoipa::{OpenApi, openapi};

#[derive(OpenApi)]
#[openapi(paths(
    get_all_post_info_handler::get_all_post_info_handler,
    get_post_by_id_handler::get_post_by_id_handler
))]
struct ApiDoc;

pub fn openapi() -> openapi::OpenApi {
    ApiDoc::openapi()
}

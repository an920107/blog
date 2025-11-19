use crate::framework::web::{get_image_by_id_handler, list_images_handler, upload_image_handler};
use utoipa::{OpenApi, openapi};

#[derive(OpenApi)]
#[openapi(paths(
    get_image_by_id_handler::get_image_by_id_handler,
    list_images_handler::list_images_handler,
    upload_image_handler::upload_image_handler
))]
struct ApiDoc;

pub fn openapi() -> openapi::OpenApi {
    ApiDoc::openapi()
}

use std::sync::Arc;

use image::{
    adapter::{
        delivery::image_controller::{ImageController, ImageControllerImpl},
        gateway::image_repository_impl::ImageRepositoryImpl,
    },
    application::use_case::{
        get_image_use_case::GetImageUseCaseImpl, upload_image_use_case::UploadImageUseCaseImpl,
    },
    framework::{
        db::image_db_service_impl::ImageDbServiceImpl,
        storage::image_storage_impl::ImageStorageImpl,
    },
};
use post::{
    adapter::{
        delivery::post_controller::{PostController, PostControllerImpl},
        gateway::post_repository_impl::PostRepositoryImpl,
    },
    application::use_case::{
        get_all_post_info_use_case::GetAllPostInfoUseCaseImpl,
        get_full_post_use_case::GetFullPostUseCaseImpl,
    },
    framework::db::post_db_service_impl::PostDbServiceImpl,
};
use sqlx::{Pool, Postgres};

pub struct Container {
    pub post_controller: Arc<dyn PostController>,
    pub image_controller: Arc<dyn ImageController>,
}

impl Container {
    pub fn new(db_pool: Pool<Postgres>, storage_path: &str) -> Self {
        let post_db_service = Arc::new(PostDbServiceImpl::new(db_pool.clone()));
        let post_repository = Arc::new(PostRepositoryImpl::new(post_db_service.clone()));
        let get_all_post_info_use_case =
            Arc::new(GetAllPostInfoUseCaseImpl::new(post_repository.clone()));
        let get_full_post_use_case = Arc::new(GetFullPostUseCaseImpl::new(post_repository.clone()));
        let post_controller = Arc::new(PostControllerImpl::new(
            get_all_post_info_use_case,
            get_full_post_use_case,
        ));

        let image_db_service = Arc::new(ImageDbServiceImpl::new(db_pool.clone()));
        let image_storage = Arc::new(ImageStorageImpl::new(storage_path.into()));
        let image_repository = Arc::new(ImageRepositoryImpl::new(
            image_db_service.clone(),
            image_storage.clone(),
        ));
        let upload_image_use_case = Arc::new(UploadImageUseCaseImpl::new(image_repository.clone()));
        let get_image_use_case = Arc::new(GetImageUseCaseImpl::new(image_repository));
        let image_controller = Arc::new(ImageControllerImpl::new(
            upload_image_use_case,
            get_image_use_case,
        ));

        Self {
            post_controller,
            image_controller,
        }
    }
}

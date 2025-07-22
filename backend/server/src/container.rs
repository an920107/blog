use std::sync::Arc;

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
}

impl Container {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        let post_db_service = Arc::new(PostDbServiceImpl::new(db_pool.clone()));

        let post_repository = Arc::new(PostRepositoryImpl::new(post_db_service.clone()));

        let get_all_post_info_use_case =
            Arc::new(GetAllPostInfoUseCaseImpl::new(post_repository.clone()));
        let get_full_post_use_case = Arc::new(GetFullPostUseCaseImpl::new(post_repository.clone()));

        let post_controller = Arc::new(PostControllerImpl::new(
            get_all_post_info_use_case,
            get_full_post_use_case,
        ));

        Self { post_controller }
    }
}

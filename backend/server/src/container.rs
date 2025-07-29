use std::sync::Arc;

use auth::{
    adapter::{
        delivery::auth_controller::{AuthController, AuthControllerImpl},
        gateway::auth_repository_impl::AuthRepositoryImpl,
    },
    application::use_case::{
        exchange_auth_code_use_case::ExchangeAuthCodeUseCaseImpl,
        get_auth_url_use_case::LoginUseCaseImpl,
    },
    framework::oidc::auth_oidc_service_impl::AuthOidcServiceImpl,
};
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
use openidconnect::reqwest;
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

use crate::configuration::Configuration;

pub struct Container {
    pub auth_controller: Arc<dyn AuthController>,
    pub image_controller: Arc<dyn ImageController>,
    pub post_controller: Arc<dyn PostController>,
}

impl Container {
    pub fn new(
        db_pool: Pool<Postgres>,
        http_client: reqwest::Client,
        configuration: Configuration,
    ) -> Self {
        let oidc_configuration = &configuration.oidc;
        let auth_oidc_service = Arc::new(AuthOidcServiceImpl::new(
            oidc_configuration.provider_metadata.clone(),
            &oidc_configuration.client_id,
            &oidc_configuration.client_secret,
            oidc_configuration.redirect_url.clone(),
            http_client,
        ));
        let auth_repository = Arc::new(AuthRepositoryImpl::new(auth_oidc_service));
        let get_auth_url_use_case = Arc::new(LoginUseCaseImpl::new(auth_repository.clone()));
        let exchange_auth_code_use_case =
            Arc::new(ExchangeAuthCodeUseCaseImpl::new(auth_repository.clone()));
        let auth_controller = Arc::new(AuthControllerImpl::new(
            get_auth_url_use_case,
            exchange_auth_code_use_case,
        ));

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
        let image_storage = Arc::new(ImageStorageImpl::new(&configuration.storage.storage_path));
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
            auth_controller,
            image_controller,
            post_controller,
        }
    }
}

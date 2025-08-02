use std::sync::Arc;

use auth::{
    adapter::{
        delivery::auth_controller::{AuthController, AuthControllerImpl},
        gateway::auth_repository_impl::AuthRepositoryImpl,
    },
    application::use_case::{
        exchange_auth_code_use_case::ExchangeAuthCodeUseCaseImpl,
        get_auth_url_use_case::LoginUseCaseImpl, get_user_use_case::GetUserUseCaseImpl,
    },
    framework::{
        db::user_db_service_impl::UserDbServiceImpl,
        oidc::auth_oidc_service_impl::AuthOidcServiceImpl,
    },
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
        gateway::{
            label_repository_impl::LabelRepositoryImpl, post_repository_impl::PostRepositoryImpl,
        },
    },
    application::use_case::{
        create_label_use_case::CreateLabelUseCaseImpl,
        get_all_labels_use_case::GetAllLabelsUseCaseImpl,
        get_all_post_info_use_case::GetAllPostInfoUseCaseImpl,
        get_full_post_use_case::GetFullPostUseCaseImpl,
        update_label_use_case::UpdateLabelUseCaseImpl,
    },
    framework::db::{
        label_db_service_impl::LabelDbServiceImpl, post_db_service_impl::PostDbServiceImpl,
    },
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
        // Auth
        let oidc_configuration = &configuration.oidc;
        let auth_oidc_service = Arc::new(AuthOidcServiceImpl::new(
            oidc_configuration.provider_metadata.clone(),
            &oidc_configuration.client_id,
            &oidc_configuration.client_secret,
            oidc_configuration.redirect_url.clone(),
            http_client,
        ));
        let user_db_service = Arc::new(UserDbServiceImpl::new(db_pool.clone()));

        let auth_repository = Arc::new(AuthRepositoryImpl::new(user_db_service, auth_oidc_service));

        let get_auth_url_use_case = Arc::new(LoginUseCaseImpl::new(auth_repository.clone()));
        let exchange_auth_code_use_case =
            Arc::new(ExchangeAuthCodeUseCaseImpl::new(auth_repository.clone()));
        let get_user_use_case = Arc::new(GetUserUseCaseImpl::new(auth_repository.clone()));

        let auth_controller = Arc::new(AuthControllerImpl::new(
            get_auth_url_use_case,
            exchange_auth_code_use_case,
            get_user_use_case,
        ));

        // Post
        let post_db_service = Arc::new(PostDbServiceImpl::new(db_pool.clone()));
        let label_db_service = Arc::new(LabelDbServiceImpl::new(db_pool.clone()));

        let post_repository = Arc::new(PostRepositoryImpl::new(post_db_service.clone()));
        let label_repository = Arc::new(LabelRepositoryImpl::new(label_db_service.clone()));

        let get_all_post_info_use_case =
            Arc::new(GetAllPostInfoUseCaseImpl::new(post_repository.clone()));
        let get_full_post_use_case = Arc::new(GetFullPostUseCaseImpl::new(post_repository.clone()));
        let create_label_use_case = Arc::new(CreateLabelUseCaseImpl::new(label_repository.clone()));
        let update_label_use_case = Arc::new(UpdateLabelUseCaseImpl::new(label_repository.clone()));
        let get_all_labels_use_case =
            Arc::new(GetAllLabelsUseCaseImpl::new(label_repository.clone()));

        let post_controller = Arc::new(PostControllerImpl::new(
            get_all_post_info_use_case,
            get_full_post_use_case,
            create_label_use_case,
            update_label_use_case,
            get_all_labels_use_case,
        ));

        // Image
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

        // Return the container with all controllers
        Self {
            auth_controller,
            image_controller,
            post_controller,
        }
    }
}

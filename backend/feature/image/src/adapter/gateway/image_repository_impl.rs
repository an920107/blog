use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapter::gateway::{
        image_db_mapper::ImageDbMapper, image_db_service::ImageDbService,
        image_storage::ImageStorage,
    },
    application::gateway::{
        create_image_params::CreateImageParams, image_repository::ImageRepository,
    },
    domain::{entity::image::Image, error::image_error::ImageError},
};

pub struct ImageRepositoryImpl {
    image_db_service: Arc<dyn ImageDbService>,
    image_storage: Arc<dyn ImageStorage>,
}

impl ImageRepositoryImpl {
    pub fn new(
        image_db_service: Arc<dyn ImageDbService>,
        image_storage: Arc<dyn ImageStorage>,
    ) -> Self {
        Self {
            image_db_service,
            image_storage,
        }
    }
}

#[async_trait]
impl ImageRepository for ImageRepositoryImpl {
    async fn save_image(&self, image: CreateImageParams) -> Result<i32, ImageError> {
        let data = image.data.clone();
        let image_id = self
            .image_db_service
            .create_image_info(ImageDbMapper {
                id: -1,
                mime_type: image.mime_type,
            })
            .await?;
        self.image_storage.write_data(image_id, &data)?;
        Ok(image_id)
    }

    async fn get_image_by_id(&self, id: i32) -> Result<Image, ImageError> {
        let image_mapper = self.image_db_service.get_image_info_by_id(id).await?;
        let data = self.image_storage.read_data(id)?;
        Ok(Image {
            id: image_mapper.id,
            mime_type: image_mapper.mime_type,
            data,
        })
    }
}

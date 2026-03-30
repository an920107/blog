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
    domain::{
        entity::{image::Image, image_meta_data::ImageMetaData},
        error::image_error::ImageError,
    },
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
        let image_db_mapper = self.image_db_service.get_image_meta_data_by_id(id).await?;
        let data = self.image_storage.read_data(id)?;
        Ok(Image {
            info: ImageMetaData {
                id: image_db_mapper.id,
                mime_type: image_db_mapper.mime_type,
            },
            data,
        })
    }

    async fn get_image_meta_data_by_id(&self, id: i32) -> Result<ImageMetaData, ImageError> {
        let image_db_mapper = self.image_db_service.get_image_meta_data_by_id(id).await?;
        Ok(ImageMetaData {
            id: image_db_mapper.id,
            mime_type: image_db_mapper.mime_type,
        })
    }

    async fn list_image_meta_data(&self) -> Result<Vec<ImageMetaData>, ImageError> {
        let image_db_mappers = self.image_db_service.list_image_meta_data().await?;

        Ok(image_db_mappers
            .into_iter()
            .map(|image_db_mapper| ImageMetaData {
                id: image_db_mapper.id,
                mime_type: image_db_mapper.mime_type,
            })
            .collect())
    }

    async fn delete_image(&self, id: i32) -> Result<(), ImageError> {
        self.image_db_service.delete_image(id).await?;
        self.image_storage.delete_data(id)?;
        Ok(())
    }
}

use std::{
    fs::{self, File},
    io::Write,
};

use crate::{
    adapter::gateway::image_storage::ImageStorage, application::error::image_error::ImageError,
};

pub struct ImageStorageImpl {
    sotrage_path: String,
}

impl ImageStorageImpl {
    pub fn new(storage_path: &str) -> Self {
        ImageStorageImpl {
            sotrage_path: storage_path.to_string(),
        }
    }
}

impl ImageStorage for ImageStorageImpl {
    fn write_data(&self, id: i32, data: &[u8]) -> Result<(), ImageError> {
        let dir_path = format!("{}/images", self.sotrage_path);
        fs::create_dir_all(&dir_path).map_err(|e| ImageError::StorageError(e.to_string()))?;

        let file_path = format!("{}/{}", dir_path, id);
        let mut file =
            File::create(&file_path).map_err(|e| ImageError::StorageError(e.to_string()))?;
        file.write_all(data)
            .map_err(|e| ImageError::StorageError(e.to_string()))?;

        Ok(())
    }

    fn read_data(&self, id: i32) -> Result<Vec<u8>, ImageError> {
        let file_path = format!("{}/images/{}", self.sotrage_path, id);
        let data = fs::read(&file_path).map_err(|e| ImageError::StorageError(e.to_string()))?;
        Ok(data)
    }
}

use crate::domain::error::image_error::ImageError;

pub trait ImageStorage: Send + Sync {
    fn write_data(&self, id: i32, data: &[u8]) -> Result<(), ImageError>;
    fn read_data(&self, id: i32) -> Result<Vec<u8>, ImageError>;
}

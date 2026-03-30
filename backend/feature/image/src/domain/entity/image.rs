use super::image_meta_data::ImageMetaData;

pub struct Image {
    pub info: ImageMetaData,
    pub data: Vec<u8>,
}

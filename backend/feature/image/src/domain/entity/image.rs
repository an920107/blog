use super::image_info::ImageInfo;

pub struct Image {
    pub info: ImageInfo,
    pub data: Vec<u8>,
}

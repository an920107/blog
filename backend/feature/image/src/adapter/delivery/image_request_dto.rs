use crate::domain::entity::image::Image;

pub struct ImageRequestDto {
    pub mime_type: String,
    pub data: Vec<u8>,
}

impl ImageRequestDto {
    pub fn into_entity(self) -> Image {
        Image {
            id: -1,
            mime_type: self.mime_type,
            data: self.data,
        }
    }
}

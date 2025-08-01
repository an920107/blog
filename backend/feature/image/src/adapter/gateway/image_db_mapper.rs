use crate::domain::entity::image::Image;

pub struct ImageDbMapper {
    pub id: i32,
    pub mime_type: String,
}

impl ImageDbMapper {
    pub fn into_entity(self) -> Image {
        Image {
            id: self.id,
            mime_type: self.mime_type,
            data: Vec::new(),
        }
    }
}

impl From<Image> for ImageDbMapper {
    fn from(image: Image) -> Self {
        ImageDbMapper {
            id: image.id,
            mime_type: image.mime_type,
        }
    }
}

use crate::application::gateway::create_image_params::CreateImageParams;

pub struct ImageRequestDto {
    pub mime_type: String,
    pub data: Vec<u8>,
}

impl From<ImageRequestDto> for CreateImageParams {
    fn from(val: ImageRequestDto) -> Self {
        CreateImageParams {
            mime_type: val.mime_type,
            data: val.data,
        }
    }
}

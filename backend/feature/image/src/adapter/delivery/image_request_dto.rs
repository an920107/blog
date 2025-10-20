use crate::application::gateway::create_image_params::CreateImageParams;

pub struct ImageRequestDto {
    pub mime_type: String,
    pub data: Vec<u8>,
}

impl Into<CreateImageParams> for ImageRequestDto {
    fn into(self) -> CreateImageParams {
        CreateImageParams {
            mime_type: self.mime_type,
            data: self.data,
        }
    }
}

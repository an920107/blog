use serde::Serialize;

use crate::domain::entity::label::Label;

#[derive(Serialize)]
pub struct LabelResponseDto {
    pub id: i32,
    pub name: String,
    pub color: String,
}

impl From<Label> for LabelResponseDto {
    fn from(entity: Label) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            color: format!("#{:08X}", entity.color),
        }
    }
}

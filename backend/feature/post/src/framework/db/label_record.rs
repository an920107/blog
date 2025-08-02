use crate::adapter::gateway::{color_db_mapper::ColorMapper, label_db_mapper::LabelMapper};

pub struct LabelRecord {
    pub id: i32,
    pub name: String,
    pub color: i64,
}

impl LabelRecord {
    pub fn into_mapper(self) -> LabelMapper {
        LabelMapper {
            id: self.id,
            name: self.name,
            color: ColorMapper {
                value: self.color as u32,
            },
        }
    }
}

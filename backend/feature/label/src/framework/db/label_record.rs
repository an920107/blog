use crate::adapter::gateway::{
    dto::color_db_model::ColorDbModel, dto::label_db_model::LabelDbModel,
};

pub struct LabelRecord {
    pub id: i32,
    pub name: String,
    pub color: i64,
}

impl LabelRecord {
    pub fn into_db_model(self) -> LabelDbModel {
        LabelDbModel {
            id: self.id,
            name: self.name,
            color: ColorDbModel {
                value: self.color as u32,
            },
        }
    }
}

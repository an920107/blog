use crate::{adapter::gateway::color_mapper::ColorMapper, domain::entity::label::Label};

pub struct LabelMapper {
    pub id: i32,
    pub name: String,
    pub color: ColorMapper,
}

impl LabelMapper {
    pub fn to_entity(&self) -> Label {
        Label {
            id: self.id,
            name: self.name.clone(),
            color: self.color.to_entity(),
        }
    }
}

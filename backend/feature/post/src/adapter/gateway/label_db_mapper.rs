use crate::{adapter::gateway::color_db_mapper::ColorMapper, domain::entity::label::Label};

pub struct LabelMapper {
    pub id: i32,
    pub name: String,
    pub color: ColorMapper,
}

impl LabelMapper {
    pub fn into_entity(self) -> Label {
        Label {
            id: self.id,
            name: self.name,
            color: self.color.into_entity(),
        }
    }
}

impl From<Label> for LabelMapper {
    fn from(label: Label) -> Self {
        Self {
            id: label.id,
            name: label.name,
            color: ColorMapper::from(label.color),
        }
    }
}

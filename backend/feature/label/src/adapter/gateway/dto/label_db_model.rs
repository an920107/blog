use crate::{adapter::gateway::dto::color_db_model::ColorDbModel, domain::entity::label::Label};

pub struct LabelDbModel {
    pub id: i32,
    pub name: String,
    pub color: ColorDbModel,
}

impl From<LabelDbModel> for Label {
    fn from(val: LabelDbModel) -> Self {
        Label {
            id: val.id,
            name: val.name,
            color: val.color.into(),
        }
    }
}

impl From<Label> for LabelDbModel {
    fn from(label: Label) -> Self {
        Self {
            id: label.id,
            name: label.name,
            color: ColorDbModel::from(label.color),
        }
    }
}

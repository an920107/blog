use crate::domain::entity::color::Color;

pub struct CreateOrUpdateLabelParams {
    pub name: String,
    pub color: Color,
}

use crate::domain::entity::color::Color;

#[derive(Clone)]
pub struct Label {
    pub id: i32,
    pub name: String,
    pub color: Color,
}

use crate::domain::entity::color::Color;

pub struct ColorMapper {
    pub value: u32,
}

impl ColorMapper {
    pub fn into_entity(self) -> Color {
        Color {
            red: (self.value >> 24) as u8,
            green: ((self.value >> 16) & 0xFF) as u8,
            blue: ((self.value >> 8) & 0xFF) as u8,
            alpha: (self.value & 0xFF) as u8,
        }
    }
}

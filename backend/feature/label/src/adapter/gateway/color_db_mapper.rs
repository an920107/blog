use crate::domain::entity::color::Color;

pub struct ColorMapper {
    pub value: u32,
}

impl From<ColorMapper> for Color {
    fn from(val: ColorMapper) -> Self {
        Color {
            red: (val.value >> 24) as u8,
            green: ((val.value >> 16) & 0xFF) as u8,
            blue: ((val.value >> 8) & 0xFF) as u8,
            alpha: (val.value & 0xFF) as u8,
        }
    }
}

impl From<Color> for ColorMapper {
    fn from(color: Color) -> Self {
        let value: u32 = ((color.red as u32) << 24)
            | ((color.green as u32) << 16)
            | ((color.blue as u32) << 8)
            | (color.alpha as u32);

        Self { value }
    }
}

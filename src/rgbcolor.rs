use super::Vec3;

#[derive(Debug, Copy, Clone, Default)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGBColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_vector(color: Vec3) -> RGBColor {
        RGBColor {
            r: (color.x * 255.99) as u8,
            g: (color.y * 255.999) as u8,
            b: (color.z * 255.999) as u8,
        }
    }
}

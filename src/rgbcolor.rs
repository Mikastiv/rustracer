use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGBColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

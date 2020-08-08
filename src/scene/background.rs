use serde::Deserialize;
use crate::rgbcolor::RGBColor;
use crate::vec3::Color;
use crate::math::clamp;

#[derive(Deserialize, Copy, Clone)]
pub struct Background {
    color1: RGBColor,
    color2: RGBColor,
}

impl Background {
    pub fn get_color(&self, alpha: f64) -> Color {
        let temp1 = (self.color2.r - self.color1.r) as f64;
        let temp2 = (self.color2.g - self.color1.g) as f64;
        let temp3 = (self.color2.b - self.color1.b) as f64;

        let r = clamp(self.color1.r + (alpha * temp1) as u8, 0, 255);
        let g = clamp(self.color1.g + (alpha * temp2) as u8, 0, 255);
        let b = clamp(self.color1.b + (alpha * temp3) as u8, 0, 255);

        Color::new(r as f64 / 255.99, g as f64 / 255.99, b as f64 / 255.99)
    }
}
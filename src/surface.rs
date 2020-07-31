use std::cmp::min;

use crate::rgbcolor::RGBColor;

pub struct Surface {
    x_offset: usize,
    y_offset: usize,
    width: usize,
    height: usize,
    buffer: Vec<RGBColor>,
}

impl Surface {
    pub fn new(x_offset: usize, y_offset: usize, width: usize, height: usize) -> Self {
        let mut buffer: Vec<RGBColor> = vec![];
        buffer.resize(width * height, RGBColor { r: 0, g: 0, b: 0 });
        Self {
            x_offset,
            y_offset,
            width,
            height,
            buffer,
        }
    }

    pub fn get_color(&self, x: usize, y: usize) -> RGBColor {
        self.buffer[x + y * self.width]
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: RGBColor) {
        self.buffer[x + y * self.width] = color;
    }

    pub fn merge(&mut self, other: &Surface) {
        let x_len = min(other.width, self.width - other.x_offset);
        let y_len = min(other.height, self.height - other.y_offset);

        for src_y in 0..y_len {
            let dst_y = other.y_offset + src_y;
            for src_x in 0..x_len {
                let dst_x = other.x_offset + src_x;
                self.buffer[dst_x + dst_y * self.width] = other.buffer[src_x + src_y * other.width];
            }
        }
    }

    pub fn save(&self, path: &str) -> image::ImageResult<()> {
        let mut img = image::ImageBuffer::new(self.width as u32, self.height as u32);
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.get_color(x, y);
                *img.get_pixel_mut(x as u32, y as u32) = image::Rgb([pixel.r, pixel.g, pixel.b]);
            }
        }

        img.save(path)
    }
}

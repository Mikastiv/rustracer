use crate::scene::Background;

#[derive(Copy, Clone)]
pub struct RenderOptions {
    pub progress_tick_rate: usize,
    pub img_width: usize,
    pub img_height: usize,
    pub sample_per_pixel: u32,
    pub max_depth: u32,
    pub background: Background,
}

impl RenderOptions {
    pub fn new(
        progress_tick_rate: usize,
        img_width: usize,
        img_height: usize,
        sample_per_pixel: u32,
        max_depth: u32,
        background: Background,
    ) -> Self {
        Self {
            progress_tick_rate,
            img_width,
            img_height,
            sample_per_pixel,
            max_depth,
            background,
        }
    }
}

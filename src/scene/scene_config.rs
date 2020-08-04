use serde::Deserialize;
use crate::vec3::Vec3;
use crate::scene::background::Background;

#[derive(Deserialize)]
pub struct SceneConfig {
    pub progress_tick_rate: usize,
    pub aspect_ratio: f64,
    pub img_width: usize,
    pub sample_per_pixel: u32,
    pub max_depth: u32,
    pub v_fov: f64,
    pub eye: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub dist_to_focus: f64,
    pub aperture: f64,
    pub background: Background,
}
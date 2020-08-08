pub mod background;
pub mod camera;
pub mod config;

use std::sync::{Arc, RwLock};

pub use self::camera::Camera;
pub use self::config::Config;
pub use self::background::Background;

use crate::hittable::HittableList;

#[derive(Clone)]
pub struct Scene {
    pub img_width: usize,
    pub img_height: usize,
    pub sample_per_pixel: u32,
    pub max_depth: u32,
    camera: Camera,
    objects: Arc<RwLock<HittableList>>,
}

impl Scene {
    pub fn new(config: &Config, objects: HittableList) -> Self {
        let camera = Camera::new(
            config.eye,
            config.look_at,
            config.up,
            config.img_width as f64 / config.img_height as f64,
            config.v_fov,
            config.aperture,
            config.dist_to_focus,
        );

        Self {
            img_width: config.img_width,
            img_height: config.img_height,
            sample_per_pixel: config.sample_per_pixel,
            max_depth: config.max_depth,
            camera,
            objects: Arc::new(RwLock::new(objects)),
        }
    }

    pub fn get_objects(&self) -> Arc<RwLock<HittableList>> {
        self.objects.clone()
    }

    pub fn get_camera(&self) -> Camera {
        self.camera.clone()
    }
}

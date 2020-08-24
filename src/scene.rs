pub mod background;
pub mod camera;
pub mod config;

pub use self::background::Background;
pub use self::camera::Camera;
pub use self::config::Config;

use crate::hittable::HittableList;

pub struct Scene {
    pub img_width: usize,
    pub img_height: usize,
    pub sample_per_pixel: u32,
    pub max_depth: u32,
    camera: Camera,
    objects: std::sync::Arc<HittableList>,
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
            config.time0,
            config.time1,
        );

        Self {
            img_width: config.img_width,
            img_height: config.img_height,
            sample_per_pixel: config.sample_per_pixel,
            max_depth: config.max_depth,
            camera,
            objects: std::sync::Arc::new(objects),
        }
    }

    pub fn get_objects(&self) -> std::sync::Arc<HittableList> {
        self.objects.clone()
    }

    pub fn get_camera(&self) -> Camera {
        self.camera.clone()
    }
}

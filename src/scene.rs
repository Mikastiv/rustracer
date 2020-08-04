pub mod background;
pub mod camera;
pub mod scene_config;

use std::sync::{Arc, RwLock};

pub use self::camera::Camera;
pub use self::scene_config::SceneConfig;

use crate::hittable_list::HittableList;

pub struct Scene {
    config: SceneConfig,
    camera: Camera,
    objects: Arc<RwLock<HittableList>>,
}

impl Scene {
    pub fn new(config: SceneConfig, camera: Camera, objects: HittableList) -> Scene {
        Scene {
            config,
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

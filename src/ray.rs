#![allow(dead_code)]
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3, time: f64) -> Self {
        Self { origin, dir, time }
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.origin + (t * self.dir)
    }
}

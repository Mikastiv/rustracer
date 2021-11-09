use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Intersection<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a Material,
}

impl Intersection<'_> {
    #[inline]
    pub fn get_face_normal(ray: Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = ray.dir.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        (front_face, normal)
    }
}

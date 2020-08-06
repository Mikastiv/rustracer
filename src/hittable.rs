use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;

#[allow(clippy::borrowed_box)]
pub struct Intersection<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a Box<dyn Material + Send + Sync + 'a>
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

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut closest = t_max;
        let mut intersection_out = None;

        for object in self.objects.iter() {
            let intersection = object.hit(ray, t_min, closest);

            intersection_out = match intersection {
                Some(hit_record) => {
                    closest = hit_record.t;
                    Some(hit_record)
                }
                None => intersection_out,
            }
        }

        intersection_out
    }
}

use crate::hittable::{Hittable, Intersection};
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
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

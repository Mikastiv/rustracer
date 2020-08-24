use std::sync::Arc;

use super::intersection::Intersection;
use super::Hittable;
use crate::axis_aligned_bb::AxisAlignedBB;
use crate::ray::Ray;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
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

    #[allow(dead_code)]
    pub fn bounding_box(&self, t0: f64, t1: f64) -> Option<AxisAlignedBB> {
        if self.objects.is_empty() {
            return None;
        };

        let mut output_box = None;

        for object in self.objects.iter() {
            let temp_box = object.bounding_box(t0, t1);

            output_box = match temp_box {
                Some(bb) => {
                    if let Some(out_bb) = output_box {
                        Some(AxisAlignedBB::surrounding_box(&out_bb, &bb))
                    } else {
                        Some(bb)
                    }
                }
                None => None,
            }
        }

        output_box
    }
}

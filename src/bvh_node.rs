use rand::{thread_rng, Rng};

use crate::axis_aligned_bb::AxisAlignedBB;
use crate::hittable::{HittableList, Intersection};
use crate::ray::Ray;

pub struct BVHNode {
    left: Box<BVHNode>,
    right: Box<BVHNode>,
    data: HittableList,
    bb: AxisAlignedBB,
    last_leaf: bool,
}

impl BVHNode {
    pub fn new(list: &HittableList, depth: u32, time0: f64, time1: f64) -> Self {
        let axis = thread_rng().gen_range(0, 3);

        let fn_comp =
            |a: &AxisAlignedBB, b: &AxisAlignedBB, axis| a.min()[axis] < b.min()[axis];

        
    }

    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        if !self.bb.hit(ray, t_min, t_max) {
            return None;
        }

        if let Some(hit_left) = self.left.hit(ray, t_min, t_max) {
            return Some(hit_left);
        }

        if let Some(hit_right) = self.right.hit(ray, t_min, t_max) {
            return Some(hit_right);
        }

        if self.last_leaf {
            self.data.hit(ray, t_min, t_max)
        } else {
            None
        }
    }

    pub fn bounding_box(&self) -> AxisAlignedBB {
        self.bb.clone()
    }
}

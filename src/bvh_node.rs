use rand::{thread_rng, Rng};

use crate::axis_aligned_bb::AxisAlignedBB;
use crate::hittable::{HittableList, Intersection, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct BVHNode {
    left: Option<Box<BVHNode>>,
    right: Option<Box<BVHNode>>,
    data: HittableList,
    bb: AxisAlignedBB,
}

impl BVHNode {
    pub fn new(
        list: HittableList,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
        last_leaf: bool,
    ) -> Self {
        let axis = thread_rng().gen_range(0, 3);
        let fn_comp = |a: &Hittable, b: &Hittable, axis| a.min()[axis] < b.min()[axis];

        let object_span = end - start;

        let mut left;
        let mut right;
        if object_span == 1 {
            left = Self::new(HittableList::new(), 0, 0, time0, time1, true);
            right = left.clone();
        } else if object_span == 2 {
            if fn_comp(list.objects[start], list.objects[start + 1]) {
                left = Self::new(HittableList::new(), 0, 0, time0, time1, true);
            }
        } else {
        }

        right
    }

    pub fn new_uninit() -> Self {
        Self {
            left: None,
            right: None,
            data: HittableList::new(),
            bb: AxisAlignedBB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
        }
    }

    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        if !self.bb.hit(ray, t_min, t_max) {
            return None;
        }
        
        if !self.data.is_empty() {
            return self.data.hit(ray, t_min, t_max);
        }

        if let Some(hit_left) = self.left.hit(ray, t_min, t_max) {
            return Some(hit_left);
        }

        if let Some(hit_right) = self.right.hit(ray, t_min, t_max) {
            return Some(hit_right);
        }

        None
    }

    pub fn bounding_box(&self) -> AxisAlignedBB {
        self.bb.clone()
    }
}

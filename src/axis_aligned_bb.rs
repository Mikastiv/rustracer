use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct AxisAlignedBB {
    min: Vec3,
    max: Vec3,
}

#[allow(dead_code)]
impl AxisAlignedBB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> bool {
        for i in 0..3 {
            let inv_div = 1.0 / ray.dir[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv_div;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv_div;

            if inv_div < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let min = if t0 > t_min { t0 } else { t_min };
            let max = if t1 < t_max { t1 } else { t_max };

            if max <= min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(bb0: &Self, bb1: &Self) -> Self {
        let small = Vec3::new(
            bb0.min().x.min(bb1.min().x),
            bb0.min().y.min(bb1.min().y),
            bb0.min().z.min(bb1.min().z),
        );
        let big = Vec3::new(
            bb0.max().x.max(bb1.max().x),
            bb0.max().y.max(bb1.max().y),
            bb0.max().z.max(bb1.max().z),
        );

        Self {
            min: small,
            max: big,
        }
    }
}

impl Default for AxisAlignedBB {
    fn default() -> Self {
        Self::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0))
    }
}

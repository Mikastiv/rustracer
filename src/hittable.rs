pub mod hittable_list;
pub mod intersection;

pub use self::hittable_list::HittableList;
pub use self::intersection::Intersection;

use crate::axis_aligned_bb::AxisAlignedBB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub enum Hittable {
    Sphere {
        center: Vec3,
        radius: f64,
        material: Material,
    },
    MovingSphere {
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Material,
    },
}

impl Hittable {
    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        match self {
            Self::Sphere {
                center,
                radius,
                material,
            } => sphere_hit(*center, *radius, material, ray, t_min, t_max),
            Self::MovingSphere {
                center0,
                center1,
                time0,
                time1,
                radius,
                material,
            } => moving_sphere_hit(*center0, *center1, *time0, *time1, *radius, material, ray, t_min, t_max)
        }
    }

    pub fn bounding_box(&self, t0: f64, t1: f64) -> Option<AxisAlignedBB> {
        match self {
            Self::Sphere {
                center,
                radius,
                material: _,
            } => {
                let bb = AxisAlignedBB::new(
                    *center - Vec3::new(*radius, *radius, *radius),
                    *center + Vec3::new(*radius, *radius, *radius),
                );
                Some(bb)
            }
            Self::MovingSphere {
                center0,
                center1,
                time0,
                time1,
                radius,
                material: _,
            } => {
                let bb0 = AxisAlignedBB::new(
                    center(*center0, *center1, *time0, *time1, t0)
                        - Vec3::new(*radius, *radius, *radius),
                    center(*center0, *center1, *time0, *time1, t0)
                        + Vec3::new(*radius, *radius, *radius),
                );

                let bb1 = AxisAlignedBB::new(
                    center(*center0, *center1, *time0, *time1, t1)
                        - Vec3::new(*radius, *radius, *radius),
                    center(*center0, *center1, *time0, *time1, t1)
                        + Vec3::new(*radius, *radius, *radius),
                );

                Some(AxisAlignedBB::surrounding_box(&bb0, &bb1))
            }
        }
    }
}

fn sphere_hit(
    center: Vec3,
    radius: f64,
    material: &Material,
    ray: Ray,
    t_min: f64,
    t_max: f64,
) -> Option<Intersection> {
    let oc = ray.origin - center;
    let a = ray.dir.length_sq();
    let half_b = oc.dot(ray.dir);
    let c = oc.length_sq() - radius.powi(2);
    let discriminant = (half_b * half_b) - (a * c);

    if discriminant > 0.0 {
        let root = discriminant.sqrt();

        {
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let point = ray.at(t);
                let outward_normal = ((point - center) / radius).normalize();
                let (front_face, normal) = Intersection::get_face_normal(ray, outward_normal);
                return Some(Intersection {
                    point,
                    normal,
                    t,
                    front_face,
                    material,
                });
            }
        }

        {
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let point = ray.at(t);
                let outward_normal = ((point - center) / radius).normalize();
                let (front_face, normal) = Intersection::get_face_normal(ray, outward_normal);
                return Some(Intersection {
                    point,
                    normal,
                    t,
                    front_face,
                    material,
                });
            }
        }
    }

    None
}

fn moving_sphere_hit(
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: &Material,
    ray: Ray,
    t_min: f64,
    t_max: f64,
) -> Option<Intersection> {
    let oc = ray.origin - center(center0, center1, time0, time1, ray.time);
    let a = ray.dir.length_sq();
    let half_b = oc.dot(ray.dir);
    let c = oc.length_sq() - radius.powi(2);
    let discriminant = (half_b * half_b) - (a * c);

    if discriminant > 0.0 {
        let root = discriminant.sqrt();

        {
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let point = ray.at(t);
                let outward_normal = ((point - center(center0, center1, time0, time1, ray.time))
                    / radius)
                    .normalize();
                let (front_face, normal) = Intersection::get_face_normal(ray, outward_normal);
                return Some(Intersection {
                    point,
                    normal,
                    t,
                    front_face,
                    material,
                });
            }
        }

        {
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let point = ray.at(t);
                let outward_normal = ((point - center(center0, center1, time0, time1, ray.time))
                    / radius)
                    .normalize();
                let (front_face, normal) = Intersection::get_face_normal(ray, outward_normal);
                return Some(Intersection {
                    point,
                    normal,
                    t,
                    front_face,
                    material,
                });
            }
        }
    }

    None
}

fn center(center0: Vec3, center1: Vec3, t0: f64, t1: f64, time: f64) -> Vec3 {
    center0 + ((time - t0) / (t1 - t0)) * (center1 - center0)
}

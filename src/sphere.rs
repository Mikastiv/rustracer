use crate::hittable::{Hittable, Intersection};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material + Send + Sync>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let oc = ray.origin - self.center;
        let a = ray.dir.length_sq();
        let half_b = oc.dot(ray.dir);
        let c = oc.length_sq() - self.radius.powi(2);
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            {
                let temp = (-half_b - root) / a;
                if temp < t_max && temp > t_min {
                    let t = temp;
                    let point = ray.at(t);
                    let outward_normal = ((point - self.center) / self.radius).normalize();
                    let (front_face, normal) = Intersection::get_face_normal(ray, outward_normal);
                    return Some(Intersection {
                        point,
                        normal,
                        t,
                        front_face,
                        material: &self.material,
                    });
                }
            }

            {
                let temp = (-half_b + root) / a;
                if temp < t_max && temp > t_min {
                    let t = temp;
                    let point = ray.at(t);
                    let outward_normal = ((point - self.center) / self.radius).normalize();
                    let (front_face, normal) = Intersection::get_face_normal(ray, outward_normal);
                    return Some(Intersection {
                        point,
                        normal,
                        t,
                        front_face,
                        material: &self.material,
                    });
                }
            }
        }

        None
    }
}

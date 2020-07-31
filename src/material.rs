use crate::hittable::Intersection;
use crate::ray::Ray;
use crate::vec3::{reflect, Color, Vec3};

pub trait Material {
    fn scatter(&self, ray: Ray, intersection: &Intersection) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        let scatter_dir = (intersection.normal + Vec3::random_unit_vector()).normalize();
        let scattered = Ray::new(intersection.point, scatter_dir);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        let reflected = reflect(ray.dir, intersection.normal);
        let scattered = Ray::new(
            intersection.point,
            reflected + self.fuzz * Vec3::random_vector_in_unit_sphere(),
        );
        let attenuation = self.albedo;

        if scattered.dir.dot(intersection.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

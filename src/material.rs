use crate::hittable::Intersection;
use crate::math::schlick;
use crate::ray::Ray;
use crate::vec3::{reflect, refract, Color, Vec3};
use rand::Rng;

pub trait Material {
    fn scatter(&self, ray: Ray, intersection: &Intersection) -> Option<(Color, Ray)>;
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        let scatter_dir = (intersection.normal + Vec3::random_unit_vector()).normalize();
        let scattered = Ray::new(intersection.point, scatter_dir);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if intersection.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let cos_t = -ray.dir.dot(intersection.normal).min(1.0);
        let sin_t = (1.0 - cos_t.powi(2)).sqrt();

        if etai_over_etat * sin_t > 1.0 {
            let reflected = reflect(ray.dir, intersection.normal);
            let scattered = Ray::new(intersection.point, reflected);
            return Some((attenuation, scattered));
        }

        let reflect_prop = schlick(cos_t, etai_over_etat);
        if rand::thread_rng().gen::<f64>() < reflect_prop {
            let reflected = reflect(ray.dir, intersection.normal);
            let scattered = Ray::new(intersection.point, reflected);
            return Some((attenuation, scattered));
        }

        let refracted = refract(ray.dir, intersection.normal, etai_over_etat);
        let scattered = Ray::new(intersection.point, refracted);
        Some((attenuation, scattered))
    }
}

use rand::Rng;

use crate::hittable::Intersection;
use crate::math::schlick;
use crate::ray::Ray;
use crate::vec3::{reflect, refract, Color, Vec3};

pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { ref_idx: f64 },
}

impl Material {
    pub fn scatter(&self, ray: Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        match self {
            Self::Lambertian { albedo } => {
                let scatter_dir = (intersection.normal + Vec3::random_unit_vector()).normalize();
                let scattered = Ray::new(intersection.point, scatter_dir);
                let attenuation = *albedo;
                Some((attenuation, scattered))
            }
            Self::Metal { albedo, fuzz } => {
                let reflected = reflect(ray.dir, intersection.normal);
                let scattered = Ray::new(
                    intersection.point,
                    reflected + *fuzz * Vec3::random_vector_in_unit_sphere(),
                );
                let attenuation = *albedo;

                if scattered.dir.dot(intersection.normal) > 0.0 {
                    Some((attenuation, scattered))
                } else {
                    None
                }
            }
            Self::Dielectric { ref_idx } => {
                let attenuation = Color::new(1.0, 1.0, 1.0);
                let etai_over_etat = if intersection.front_face {
                    1.0 / *ref_idx
                } else {
                    *ref_idx
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
    }
}

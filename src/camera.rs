use crate::math;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        eye: Vec3,
        lookat: Vec3,
        up: Vec3,
        aspect_ratio: f64,
        vfov: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = math::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (eye - lookat).normalize();
        let u = (up.cross(w)).normalize();
        let v = w.cross(u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        Camera {
            origin: eye,
            horizontal,
            vertical,
            lower_left_corner: eye - horizontal / 2.0 + vertical / 2.0 - focus_dist * w,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_vector_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin, /*+ offset*/
            self.lower_left_corner + s * self.horizontal - t * self.vertical - self.origin, /*- offset*/
        )
    }
}
